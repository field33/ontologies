
# Change Log
All notable changes to this project will be documented in this file.
 
The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

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