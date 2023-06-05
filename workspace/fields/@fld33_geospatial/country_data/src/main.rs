pub mod common;

use crate::common::{
    countries_to_retrieve, Country, CountryWithRegion, MUNICIPALITIES_OUTPUT_PATH,
};
use anyhow::{bail, Context, Error};
use duct::cmd;
use kdam::term::Colorizer;
use kdam::tqdm;
use rio_api::model::Subject;
use rio_api::parser::TriplesParser;
use rio_turtle::TurtleParser;
use std::collections::HashMap;
use std::io::{BufReader, Cursor};
use std::path::{Path, PathBuf};
use tempdir::TempDir;

const SPARQL_ENDPOINT_WIKIDATA: &str = "https://query.wikidata.org/sparql";
const BIN_PATH_JENA_RSPARQL: &str = "./bin/apache-jena/bin/rsparql";

pub fn jena_rsparql_path() -> Result<PathBuf, Error> {
    let path = PathBuf::from(BIN_PATH_JENA_RSPARQL).canonicalize()?;
    if !path.exists() {
        bail!("`rsparql` binary not found at path `{path}`. Please download via `just setup_jena` first.", path = path.display());
    }
    Ok(path)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let countries = countries_to_retrieve();
    let mut countries_with_regions = vec![];
    for country in tqdm!(
        countries.iter(),
        total = countries.len(),
        desc = "Retrieving regions for countries "
    ) {
        let get_regions_query = build_query_regions_per_country(&country.wikidata_country_entity)?;
        let regions_for_country_result = query_data_from_wikidata(&get_regions_query.0).await?;

        let region_iris = extract_region_iris_from_result(&regions_for_country_result)?;

        countries_with_regions.append(
            &mut region_iris
                .into_iter()
                .map(|region_iri| country.with_region(region_iri))
                .collect::<Vec<_>>(),
        );
    }

    for country_with_region in tqdm!(
        countries_with_regions.iter(),
        total = countries_with_regions.len(),
        desc = "Retrieving municipalities for countries + regions "
    ) {
        if output_path(country_with_region)?.exists() {
            println!(
                "Skipping {country} {region} as the output file already exists",
                country = &country_with_region.wikidata_country_entity,
                region = country_with_region.wikidata_region_entity_id()?
            );
            continue;
        }

        let built_query = build_query_per_region(
            &country_with_region.wikidata_country_entity,
            &country_with_region.wikidata_region_iri,
            &country_with_region.wikidata_municipality_key_property,
        )?;
        let query = &built_query.0;
        let result = query_data_from_wikidata(query).await?;
        write_region_results(country_with_region, &result)?;
    }

    Ok(())
}

pub fn output_path(country_with_region: &CountryWithRegion) -> Result<PathBuf, Error> {
    Ok(Path::new(MUNICIPALITIES_OUTPUT_PATH)
        .join(&country_with_region.wikidata_country_entity)
        .join(&format!(
            "{}.ttl",
            country_with_region.wikidata_region_entity_id()?
        )))
}

pub fn write_region_results(
    country_with_region: &CountryWithRegion,
    result: &NTriplesString,
) -> Result<(), Error> {
    let output_path = output_path(country_with_region)?;
    std::fs::create_dir_all(
        output_path
            .parent()
            .context("Output path has no parent path")?,
    )?;
    std::fs::write(output_path, &result.0)?;
    Ok(())
}

#[derive(Debug)]
pub struct NTriplesString(String);

async fn query_data_from_wikidata(query: &str) -> Result<NTriplesString, Error> {
    query_triples_from_sparql_endpoint(SPARQL_ENDPOINT_WIKIDATA, query).await
}

async fn query_triples_from_sparql_endpoint(
    endpoint: &str,
    query: &str,
) -> Result<NTriplesString, Error> {
    query_triples_via_jena(endpoint, query).await
}

async fn query_triples_via_jena(endpoint: &str, query: &str) -> Result<NTriplesString, Error> {
    let binary_path = jena_rsparql_path()?;
    let temp_dir = TempDir::new("sparql_query")?;
    let query_path = temp_dir.path().join("query.rq");

    std::fs::write(&query_path, query)?;

    let command = cmd(
        binary_path,
        &[
            "--service",
            endpoint,
            "--query",
            &query_path.to_string_lossy(),
            // List of possible `results` values: https://github.com/apache/jena/blob/6cb004690f1d2342c98eeee06685276ac4e5bc73/jena-arq/src/main/java/org/apache/jena/sparql/resultset/ResultsFormat.java#L82-L117
            "--results",
            "ntriples",
        ],
    )
    .stdout_capture();

    let command_result = command.run()?;
    let output = String::from_utf8(command_result.stdout)?;
    Ok(NTriplesString(output))
}

#[derive(Debug)]
pub struct SparqlQueryString(String);

/// We have to do a query per country and region to stay below hard-coded 10k row limit of Wikidata
///
/// example: `wikidata_country_entity` = `"Q183"` (Germany)
/// example: `wikidata_region_entity` = `"Q980"` (Bavaria)
/// example: `wikidata_municipality_key_property` = `"P439"` ("German municipality key")
fn build_query_per_region(
    wikidata_country_entity: &str,
    wikidata_region_iri: &str,
    wikidata_municipality_key_property: &str,
) -> Result<SparqlQueryString, Error> {
    let construct_template = format!(
        "\
          ?countryIri a owl:NamedIndividual .
          ?countryIri a <http://field33.com/ontologies/@fld33_domain/geospatial/Country> .
          ?countryIri rdfs:label ?countryLabel_de .

          ?regionIri a owl:NamedIndividual .
          ?regionIri a <http://field33.com/ontologies/@fld33_domain/geospatial/Region> .
          ?regionIri rdfs:label ?regionLabel_de .
          ?regionIri <http://field33.com/ontologies/@fld33_domain/geospatial/RegionPartOfCountry> ?countryIri .

          ?municipalityIri a owl:NamedIndividual .
          ?municipalityIri a <http://field33.com/ontologies/@fld33_domain/geospatial/Municipality> .
          ?municipalityIri rdfs:label ?municipalityLabel_de .
          ?municipalityIri <http://field33.com/ontologies/@fld33_domain/geospatial/MunicipalityPartOfRegion> ?regionIri .
    "
    );
    // List of prefixes from <https://www.wikidata.org/wiki/EntitySchema:E49>
    let query_string = format!("\
        PREFIX bd: <http://www.bigdata.com/rdf#>
        PREFIX cc: <http://creativecommons.org/ns#>
        PREFIX dct: <http://purl.org/dc/terms/>
        PREFIX geo: <http://www.opengis.net/ont/geosparql#>
        PREFIX ontolex: <http://www.w3.org/ns/lemon/ontolex#>
        PREFIX owl: <http://www.w3.org/2002/07/owl#>
        PREFIX p: <http://www.wikidata.org/prop/>
        PREFIX pq: <http://www.wikidata.org/prop/qualifier/>
        PREFIX pqn: <http://www.wikidata.org/prop/qualifier/value-normalized/>
        PREFIX pqv: <http://www.wikidata.org/prop/qualifier/value/>
        PREFIX pr: <http://www.wikidata.org/prop/reference/>
        PREFIX prn: <http://www.wikidata.org/prop/reference/value-normalized/>
        PREFIX prov: <http://www.w3.org/ns/prov#>
        PREFIX prv: <http://www.wikidata.org/prop/reference/value/>
        PREFIX ps: <http://www.wikidata.org/prop/statement/>
        PREFIX psn: <http://www.wikidata.org/prop/statement/value-normalized/>
        PREFIX psv: <http://www.wikidata.org/prop/statement/value/>
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
        PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
        PREFIX schema: <http://schema.org/>
        PREFIX skos: <http://www.w3.org/2004/02/skos/core#>
        PREFIX wd: <http://www.wikidata.org/entity/>
        PREFIX wdata: <http://www.wikidata.org/wiki/Special:EntityData/>
        PREFIX wdno: <http://www.wikidata.org/prop/novalue/>
        PREFIX wdref: <http://www.wikidata.org/reference/>
        PREFIX wds: <http://www.wikidata.org/entity/statement/>
        PREFIX wdt: <http://www.wikidata.org/prop/direct/>
        PREFIX wdtn: <http://www.wikidata.org/prop/direct-normalized/>
        PREFIX wdv: <http://www.wikidata.org/value/>
        PREFIX wikibase: <http://wikiba.se/ontology#>
        PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

        CONSTRUCT {{
            {construct_template}
        }}
        WHERE {{
            # List of regions, whose sub-regions we want
            VALUES ?country {{
              wd:{wikidata_country_entity}
              # Germany
              # wd:Q183
              # France
              # wd:Q142
              # Austria
              # wd:Q40
            }}
            VALUES ?region {{
              # Bavaria
             # wd:Q980
             <{wikidata_region_iri}>
            }}
          
            BIND(\"http://field33.com/ontologies/@fld33_domain/geospatial/\" AS ?iriBase)
          
            # Get label, ISO code for country
            OPTIONAL {{ ?country rdfs:label ?countryLabel_de . FILTER(LANG(?countryLabel_de) = \"de\") }}
            ?country wdt:P297 ?countryIso3166_1_twoLetter
            BIND(IRI(CONCAT(?iriBase, \"country/\", ?countryIso3166_1_twoLetter)) AS ?countryIri)
                     
            # P150 = 'contains administrative territorial entity'
            # but must not have a P582 (end date) qualifier
            ?country p:P150 ?statement .
            ?statement ps:P150 ?region .
            FILTER NOT EXISTS {{ ?statement pq:P582 ?x }}
            # Get label, ISO code for region
            OPTIONAL {{ ?region rdfs:label ?regionLabel_de . FILTER(LANG(?regionLabel_de) = \"de\") }}
            ?region wdt:P300 ?regionIso_3166_2
            BIND(IRI(CONCAT(?iriBase, \"region/\", ?regionIso_3166_2)) AS ?regionIri)
                    
            {{
              SELECT ?country ?municipality (SAMPLE(?municipalityKeyInSubQuery) AS ?municipalityKey) WHERE {{
                 ?municipality wdt:{wikidata_municipality_key_property} ?municipalityKeyInSubQuery .

                BIND(wd:{wikidata_country_entity} AS ?country)
              }}
              GROUP BY ?country ?municipality    
            }}
            ?municipality wdt:P131?/wdt:P131?/wdt:P131? ?region .
            # municipality key is uniquer per country
            BIND(IRI(CONCAT(STR(?countryIri), \"/municipality/\", ?municipalityKey)) AS ?municipalityIri)
            OPTIONAL {{ ?municipality rdfs:label ?municipalityLabel_de . FILTER(LANG(?municipalityLabel_de) = \"de\") }}
        }}
    ",
       construct_template = construct_template,
       wikidata_country_entity = wikidata_country_entity,
       wikidata_region_iri = wikidata_region_iri,
       wikidata_municipality_key_property = wikidata_municipality_key_property,
    );

    Ok(SparqlQueryString(query_string))
}

/// example: `wikidata_country_entity` = `"Q183"` (Germany)
fn build_query_regions_per_country(
    wikidata_country_entity: &str,
) -> Result<SparqlQueryString, Error> {
    // List of prefixes from <https://www.wikidata.org/wiki/EntitySchema:E49>
    let query_string = format!(
        "\
        PREFIX bd: <http://www.bigdata.com/rdf#>
        PREFIX cc: <http://creativecommons.org/ns#>
        PREFIX dct: <http://purl.org/dc/terms/>
        PREFIX geo: <http://www.opengis.net/ont/geosparql#>
        PREFIX ontolex: <http://www.w3.org/ns/lemon/ontolex#>
        PREFIX owl: <http://www.w3.org/2002/07/owl#>
        PREFIX p: <http://www.wikidata.org/prop/>
        PREFIX pq: <http://www.wikidata.org/prop/qualifier/>
        PREFIX pqn: <http://www.wikidata.org/prop/qualifier/value-normalized/>
        PREFIX pqv: <http://www.wikidata.org/prop/qualifier/value/>
        PREFIX pr: <http://www.wikidata.org/prop/reference/>
        PREFIX prn: <http://www.wikidata.org/prop/reference/value-normalized/>
        PREFIX prov: <http://www.w3.org/ns/prov#>
        PREFIX prv: <http://www.wikidata.org/prop/reference/value/>
        PREFIX ps: <http://www.wikidata.org/prop/statement/>
        PREFIX psn: <http://www.wikidata.org/prop/statement/value-normalized/>
        PREFIX psv: <http://www.wikidata.org/prop/statement/value/>
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
        PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
        PREFIX schema: <http://schema.org/>
        PREFIX skos: <http://www.w3.org/2004/02/skos/core#>
        PREFIX wd: <http://www.wikidata.org/entity/>
        PREFIX wdata: <http://www.wikidata.org/wiki/Special:EntityData/>
        PREFIX wdno: <http://www.wikidata.org/prop/novalue/>
        PREFIX wdref: <http://www.wikidata.org/reference/>
        PREFIX wds: <http://www.wikidata.org/entity/statement/>
        PREFIX wdt: <http://www.wikidata.org/prop/direct/>
        PREFIX wdtn: <http://www.wikidata.org/prop/direct-normalized/>
        PREFIX wdv: <http://www.wikidata.org/value/>
        PREFIX wikibase: <http://wikiba.se/ontology#>
        PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

        CONSTRUCT {{
         ?region a owl:NamedIndividual .
        }}
        WHERE {{
            # List of country, whose sub-regions we want
            VALUES ?country {{
              wd:{wikidata_country_entity}
              # Germany
              # wd:Q183
            }}
            # P150 = 'contains administrative territorial entity'
            # but must not have a P582 (end date) qualifier
            ?country p:P150 ?statement .
            ?statement ps:P150 ?region .
            FILTER NOT EXISTS {{ ?statement pq:P582 ?x }}
        }}
    ",
        wikidata_country_entity = wikidata_country_entity,
    );

    Ok(SparqlQueryString(query_string))
}

fn extract_region_iris_from_result(result: &NTriplesString) -> Result<Vec<String>, Error> {
    let mut region_iris = vec![];
    TurtleParser::new(Cursor::new(&result.0), None).parse_all(&mut |t| {
        if let Subject::NamedNode(region_iri) = t.subject {
            region_iris.push(region_iri.iri.to_string());
        }
        Ok::<_, Error>(())
    })?;

    Ok(region_iris)
}
