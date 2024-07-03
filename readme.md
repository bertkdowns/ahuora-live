# Ahuora-Live: SDK/Platform for using the Ahuora Simulation platform for live-streaming

In its initial stages, this will simply involve using the ahuora backend endpoints to set PropertyInfo values by unit op name and propkey type. However, eventually I plan to add a specific solve endpoint that automatically sets the required propertyInfo values before sending to IDAES.

To access the api in rust, openapi-generator is used to generate a rust client.

