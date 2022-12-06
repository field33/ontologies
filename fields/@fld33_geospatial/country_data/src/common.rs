use anyhow::{Context, Error};

pub const DATASET_BASE_IRI: &str = "http://field33.com/ontologies/@fld33_domain/geospatial/";
pub const MUNICIPALITIES_OUTPUT_PATH: &str = "./out/wikidata_municipalities/";

pub fn countries_to_retrieve() -> Vec<Country> {
    vec![
        // Germany
        Country::new("DE", "Q183", "P439"),
        // Austria
        Country::new("AT", "Q40", "P964"),
        // Poland
        // TODO: the values of P1653 look to deviate from the LAU codes according to the Eurostat table
        Country::new("PL", "Q36", "P1653"),
        // France
        Country::new("FR", "Q142", "P374"),
        // Switzerland
        Country::new("CH", "Q39", "P771"),
        // Netherlands
        Country::new("NL", "Q55", "P382"),
        // Denmark
        Country::new("DK", "Q35", "P1168"),
        // Belgium
        Country::new("BE", "Q31", "P1567"),
    ]
}

#[derive(Debug)]
pub struct Country {
    pub iso_two_letter: String,
    pub wikidata_country_entity: String,
    pub wikidata_municipality_key_property: String,
}

impl Country {
    pub fn new<S1: Into<String>, S2: Into<String>, S3: Into<String>>(
        iso_two_letter: S1,
        wikidata_country_entity: S2,
        wikidata_municipality_key_property: S3,
    ) -> Self {
        Self {
            iso_two_letter: iso_two_letter.into(),
            wikidata_country_entity: wikidata_country_entity.into(),
            wikidata_municipality_key_property: wikidata_municipality_key_property.into(),
        }
    }

    pub fn with_region<S: Into<String>>(&self, region_iri: S) -> CountryWithRegion {
        CountryWithRegion {
            wikidata_country_entity: self.wikidata_country_entity.clone(),
            wikidata_municipality_key_property: self.wikidata_municipality_key_property.clone(),
            wikidata_region_iri: region_iri.into(),
        }
    }
}

#[derive(Debug)]
pub struct CountryWithRegion {
    pub wikidata_country_entity: String,
    pub wikidata_municipality_key_property: String,
    pub wikidata_region_iri: String,
}

impl CountryWithRegion {
    pub fn wikidata_region_entity_id(&self) -> Result<String, Error> {
        let parts = self.wikidata_region_iri.split("/");
        let entity_id = parts.last().context("Region IRI has no last part")?;
        Ok(entity_id.to_string())
    }
}
