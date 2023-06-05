use crate::common::{countries_to_retrieve, Country, MUNICIPALITIES_OUTPUT_PATH};
use anyhow::Error;
use itertools::Itertools;
use oxigraph;
use oxigraph::io::{GraphFormat, GraphParser, GraphSerializer};
use oxigraph::model::Triple;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};

pub mod common;

const GENERATED_ONTOLOGIES_VERSION: &str = "0.1.0";

fn build_country_file_header(country: &Country) -> String {
    format!(
        r#"@prefix : <http://field33.com/ontologies/@fld33_geospatial/{iso_two_letter}/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix xml: <http://www.w3.org/XML/1998/namespace> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix registry: <http://field33.com/ontologies/REGISTRY/> .
@base <http://field33.com/ontologies/@fld33_geospatial/{iso_two_letter}/> .

<http://field33.com/ontologies/@fld33_geospatial/{iso_two_letter}/> rdf:type owl:Ontology ;
    registry:author "Field 33 <contribution@field33.com>" ;
registry:category "Core" ;
registry:dependency "@fld33_domain/geospatial ^0.1.0" ;
registry:keyword "Field 33 Package" ;
registry:licenseSPDX "CC0-1.0" ;
registry:ontologyFormatVersion "v1" ;
registry:packageName "@fld33_geospatial/{iso_two_letter}" ;
registry:packageVersion "{package_version}" ;
registry:repository "https://github.com/field33/ontologies/tree/main/%40fld33_geospatial" ;
registry:shortDescription "Geospatial upper ontology"@en ;
rdfs:comment "Geospatial data ontology for country {iso_two_letter}."@en ;
rdfs:label "Geospatial data ontology for country {iso_two_letter}"@en .

"#,
        iso_two_letter = country.iso_two_letter,
        package_version = GENERATED_ONTOLOGIES_VERSION,
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    for country in countries_to_retrieve() {
        let country_output_directory =
            Path::new(MUNICIPALITIES_OUTPUT_PATH).join(&country.wikidata_country_entity);

        let mut all_triples = vec![];
        for file in std::fs::read_dir(country_output_directory)? {
            let mut triples = read_triple_file(file?.path())?;
            all_triples.append(&mut triples);
        }

        let all_triples = all_triples.into_iter().unique().collect();

        let output_path = Path::new("./out/datasets_per_country/")
            .join(&format!("{}.ttl", &country.iso_two_letter));
        std::fs::create_dir_all(output_path.parent().unwrap())?;
        write_triple_file(output_path, all_triples, &country)?;
    }

    Ok(())
}

fn read_triple_file<P: Into<PathBuf>>(path: P) -> Result<Vec<Triple>, Error> {
    let path = path.into();

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let parser = GraphParser::from_format(GraphFormat::Turtle);

    let triples = parser
        .read_triples(reader)?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(triples)
}

fn write_triple_file<P: Into<PathBuf>>(
    path: P,
    triples: Vec<Triple>,
    country: &Country,
) -> Result<(), Error> {
    let path = path.into();

    let mut buffer = Vec::new();
    buffer.write(build_country_file_header(country).as_bytes())?;
    let mut writer =
        GraphSerializer::from_format(GraphFormat::Turtle).triple_writer(&mut buffer)?;
    for triple in triples {
        writer.write(&triple)?;
    }

    std::fs::write(path, buffer)?;

    Ok(())
}
