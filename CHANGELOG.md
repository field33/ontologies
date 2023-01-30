
# Change Log
All notable changes to this project will be documented in this file.
 
The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [*] - 2022-01-13
`@w3c_mirror/prov-o/prov-o`
`@contrib/generic_innovation_interoperability/generic_innovation_interoperability`
`@apqc/apqc_aerospace_and_defence/apqc_aerospace_and_defence`
`@apqc/apqc/apqc`
`@apqc/apqc_airline/apqc_airline`
`@fld33_domain/software_architecture_metric/software_architecture_metric`
`@fld33_domain/software_team_metric/software_team_metric`
`@fld33_domain/software_development_lifecycle/software_development_lifecycle`
`@fld33_domain/business_objects/business_objects`
`@fld33_domain/software_development_metric/software_development_metric`
`@fld33_domain/software_development/software_development`
`@fld33_domain/software_implementation_metric/software_implementation_metric`
`@fld33_domain/geospatial/geospatial`
`@foaf_mirror/foaf/foaf`
`@apqc_cross_industry/market_and_sell_products_and_services/market_and_sell_products_and_services`
`@apqc_cross_industry/develop_and_manage_human_capital/develop_and_manage_human_capital`
`@apqc_cross_industry/manage_information_technology_it/manage_information_technology_it`
`@apqc_cross_industry/manage_external_relationships/manage_external_relationships`
`@apqc_cross_industry/manage_financial_resources/manage_financial_resources`
`@apqc_cross_industry/develop_and_manage_products_and_services/develop_and_manage_products_and_services`
`@apqc_cross_industry/manage_enterprise_risk_compliance_remediation_and_resiliency/manage_enterprise_risk_compliance_remediation_and_resiliency`
`@apqc_cross_industry/develop_vision_and_strategy/develop_vision_and_strategy`
`@apqc_cross_industry/develop_and_manage_business_capabilities/develop_and_manage_business_capabilities`
`@apqc_cross_industry/manage_customer_service/manage_customer_service`
`@apqc_cross_industry/apqc_cross_industry/apqc_cross_industry`
`@apqc_cross_industry/acquire_construct_and_manage_assets/acquire_construct_and_manage_assets`
`@apqc_cross_industry/deliver_services/deliver_services`
`@apqc_cross_industry/manage_supply_chain_for_physical_products/manage_supply_chain_for_physical_products`
`@fld33/organization/organization`
`@fld33/relations/relations`
`@fld33/methodology/methodology`
`@fld33/organization_communication/organization_communication`
`@fld33/people/people`
`@fld33/communication/communication`
`@fld33/process/process`
 
### Added

### Changed
- Updated repository linkings and bumped all versions
### Fixed

## [^0.1.10] - 2022-01-13
`@fld33_domain/software_development`
 
### Added
- Object property: `DeploymentPartOfTicket`
### Changed

### Fixed
## [^0.1.10] - 2022-01-13
`@fld33_domain/business_objects`
 
### Added
- Classes: `Capability` as subClass of `BusinessObject`
### Changed

### Fixed

## [^0.2.1] - 2022-01-13
`@fld33/relations`
 
### Added
- Object Properties: `RelatesTo`
### Changed

### Fixed

## [^0.2.7] - 2022-01-13
`@fld33_domain/software_development`
 
### Added
- Object Properties: `BlockedBy`, `ExtensionOf` with Domain Tickets for Range `Functionality` and `SoftwareCapability` via respective SubProperties
- Classes: `Functionality`, `SoftwareCapability`, `Feature` were added
### Changed
- Dependency update: `@fld33/relations ^0.2.1` version bumped up

### Fixed

## [^0.2.4] - 2022-12-16
`@fld33_domain/software_development`
 
### Added
- Object Properties: `AssignedTo`, `CreatedBy` with Domain Tickets for Range `Team` and `People` via respective SubProperties
- New dependencies: `@fld33/people ^0.1.5` and `@fld33/organization ^0.1.4` added to `@fld33_domain/software_development`
### Changed

### Fixed

## [^0.1.0] - 2022-12-07
This change introduces the @fld33_domain/geospatial Field in version 0.1.0
 
### Added
- @fld33_domain/geospatial (basic classes: city, municipality, region, country, continent as well as partOf object property relations)
### Changed

### Fixed

## [init] - 2022-11-07
This change is the initial changelog message with a lot of refactoring.
 
### Added

### Changed
- update `Plow.lock`
- Add Readme
- Add Changelog
- Adjusted file naming

### Fixed
- wrong markdown formatting for headlines
- Updated namespaces for `apqc_aerospace_and_defense` and `apqc_airline`