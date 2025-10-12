# Start from the official Rust Dev Container base image
FROM mcr.microsoft.com/devcontainers/rust:1

# Set working directory
WORKDIR /workspace

# Install system dependencies
RUN apt-get update && \
    apt-get install -y \
        build-essential \
        pkg-config \
        libssl-dev \
        python3 \
        python3-pip \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Copy project files into container
COPY . .

# Install Python requirements if the file exists
RUN if [ -f build/requirements.txt ]; then pip install -r build/requirements.txt; fi

# Install Rust components
RUN rustup component add rustfmt clippy

# Build Rust project
RUN cargo build

# Download and run the Luajit installer
RUN curl -L -o "LuajitInstall.sh" "https://raw.githubusercontent.com/ShadowDara/LuaAPI-Rust/refs/heads/main/installer/install.sh" && \
    chmod +x LuajitInstall.sh && \
    ./LuajitInstall.sh "v0.1.13"

# Run a LuaJIT test (optional – only works if `luajit` is correctly installed and build.lua exists)
RUN if [ -f build.lua ]; then luajit build.lua; fi
