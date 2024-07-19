# Ahuora-Live: SDK/Platform for using the Ahuora Simulation platform for live-streaming

In its initial stages, this will simply involve using the ahuora backend endpoints to set PropertyInfo values by unit op name and propkey type. However, eventually I plan to add a specific solve endpoint that automatically sets the required propertyInfo values before sending to IDAES.

To access the api in rust, openapi-generator is used to generate a rust client. Note that the current openapi-generator doesn't support this, so I've had to pull and build the generator myself to implement the pull request.

https://github.com/OpenAPITools/openapi-generator/pull/19199


Checklist To Do next:
 - [] Architecture: 
   - [] All the stuff for connecting to the Ahuora platform should be seperated from the core logic.
   - [] All the stuff for connecting to a data source (e.g csv file, influxdb server, kafka or mqtt pipeline) should be abstracted into a seperate module/trait
 - [] Ideally, tests should cover the core logic or something
 - [] We need a better way of setting up custom configuration (either with a config file, or turning the whole thing into a library and making a seperate binary crate for executing it)
 - [] We need to figure out how often to re-solve or re-calculate