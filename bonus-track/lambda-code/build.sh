#!/bin/bash

# Cleanup previous builds
rm ../build/bootstrap.zip || true

BUILD_IMAGE="rustserverless/lambda-rust"  # https://github.com/rust-serverless/lambda-rust

# x64 values
# PLATFORM="linux/amd64"
# TAG="latest"

# ARM values
PLATFORM="linux/arm64"
TAG="latest-arm64"

# This will build the binary and the zip file with it inside necessary for Lambda
docker run --rm \
  --platform ${PLATFORM} \
  -u "$(id -u)":"$(id -g)" \
  -v ${PWD}:/code \
  -v ${HOME}/.cargo/registry:/cargo/registry \
  -v ${HOME}/.cargo/git:/cargo/git \
  ${BUILD_IMAGE}:${TAG}

# Create the target dir if it does not exists
mkdir ../build/ || true
# Then we place the output into the folder we use in CDK to fetch the Lambda asset
mv "target/lambda/release/bootstrap.zip" ../build/bootstrap.zip
