
# For this script to work, openapi generator must be pulled.
# This command is setup for the openapi generator to be pulled from the bertkdowns/openapi-generator repository, and placed in a folder adjacent to ahuora-live.
# Then built as per the repo instructions.
# Likewise, the Ahuora Adaptive Digital Twin platform must be pulled in the same way.

java -jar ../openapi-generator/modules/openapi-generator-cli/target/openapi-generator-cli.jar generate -i ../Ahuora-Adaptive-Digital-Twin-Platform/FrontEnd/src/api/api_schema.yml -g rust -o openapi