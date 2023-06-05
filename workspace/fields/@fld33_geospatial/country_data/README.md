# `@fld33_geospatial` country ontologies

This is the code to gather data from Wikidata to assemble the `@fld33_geospatial` for European countries
(e.g. `@fld33_geospatial/DE`).

The ontologies contains the following data:
- `Country`s, based on [Eurostat NUTS 0](https://en.wikipedia.org/wiki/Nomenclature_of_Territorial_Units_for_Statistics)
- `Region`s, based on `NUTS 1`
- `Municipality`s, based on the Eurostat LAU classification
- The `de` labels for all of the above
- The connections for the next-higher level for all of the above

## Development build

To gather the data from Wikdiata:
```shell
just retrieve_wikidata
```

To assemble the data into ontologies with the appropriate metadata:
```
just merge
```