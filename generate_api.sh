
# For this script to work, openapi generator must be installed. You can download and install the binaries manually,
# or install the command with npm
# npm install @openapitools/openapi-generator-cli -g
# openapi-generator-cli version


openapi-generator-cli generate -i ../Ahuora-Adaptive-Digital-Twin-Platform/FrontEnd/src/api/api_schema.yml -g rust -o openapi