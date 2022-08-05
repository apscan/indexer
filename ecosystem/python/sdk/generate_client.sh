#!/bin/sh

cd "$(dirname "$0")"

# This docker command:
# - Mounts the spec into the container.
# - Mounts the output directory for the generated code.
# - Uses the new experimental Python client generator.
docker run \
    --rm \
    -it \
    --mount type=bind,source=`realpath ../../../api/doc/v1/spec.yaml`,target=/spec.yaml \
    --mount type=bind,source=`realpath ./`,target=/output \
    openapitools/openapi-generator-cli:v6.0.1 \
    generate \
    -c /output/openapi_generator_config.json \
    -g python-experimental \
    -i /spec.yaml \
    -o /output

# Remove unnecessary generated files.
rm .gitlab-ci.yml .travis.yml git_push.sh
