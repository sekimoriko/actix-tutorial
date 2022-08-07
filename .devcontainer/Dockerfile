# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.241.1/containers/rust/.devcontainer/base.Dockerfile

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="buster"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT}

# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>

# https://github.com/microsoft/vscode-dev-containers/blob/main/script-library/docs/docker.md#script-use
COPY library-scripts/docker-debian.sh /tmp/library-scripts/
ENV DOCKER_BUILDKIT=1
RUN apt-get update && bash /tmp/library-scripts/docker-debian.sh
ENTRYPOINT ["/usr/local/share/docker-init.sh"]
CMD ["sleep", "infinity"]
