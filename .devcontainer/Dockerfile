FROM rust:1-slim

# Install dependencies
RUN /bin/sh -c set -eux; \
    apt-get update; \
    DEBIAN_FRONTEND=noninteractive apt-get -y install --no-install-recommends \
        ca-certificates curl git gnupg2 locales locales-all wget zsh; \
    rm -rf /var/lib/apt/lists/*

# Create local user
RUN useradd -ms /bin/zsh vscode

# Switch to local user
USER vscode

# Install and configure zsh
RUN sh -c "$(wget -O- https://github.com/deluan/zsh-in-docker/releases/download/v1.2.1/zsh-in-docker.sh)" -- \
        -x \
        -t robbyrussell \
        -p debian \
        -p gitignore \
        -p rust
