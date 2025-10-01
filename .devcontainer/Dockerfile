FROM rust:1.90-slim-trixie

RUN apt-get update && apt-get install -y \
        curl \
        xz-utils \
        git \
        libudev-dev \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/download/v0.29.1/probe-rs-tools-installer.sh | sh

RUN mkdir -p /workspaces/miratherm-radiator-thermostat-software