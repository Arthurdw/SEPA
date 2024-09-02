#!/bin/bash

set -e

# Define your Docker Hub repository and image name
DOCKER_REPO="arthurdw/sepa"
DOCKER_TAG="latest"

# Extract version from Cargo.toml
VERSION=$(grep '^version' Cargo.toml | awk -F '= ' '{print $2}' | tr -d ' ')

if [ -z "$VERSION" ]; then
  echo "Error: VERSION is empty"
  exit 1
else
  # Replace invalid characters for Docker tags
  VERSION=$(echo "$VERSION" | tr -d '"')
  VERSION=$(echo "$VERSION" | tr '[:upper:]' '[:lower:]') # Ensure lowercase
fi

# Build the Docker image
echo "Building Docker image..."
docker build -t "${DOCKER_REPO}:${VERSION}" .

# Tag the image as 'latest'
docker tag "${DOCKER_REPO}:${VERSION}" "${DOCKER_REPO}:${DOCKER_TAG}"

# Push the Docker image
echo "Pushing Docker image to Docker Hub..."
docker push "${DOCKER_REPO}:${VERSION}"
docker push "${DOCKER_REPO}:${DOCKER_TAG}"

echo "Docker image published successfully."
