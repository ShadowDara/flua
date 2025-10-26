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

# Install Python requirements
RUN apt-get update && apt-get install -y python3-venv
RUN python3 -m venv /opt/venv
ENV PATH="/opt/venv/bin:$PATH"
RUN pip install -r build/requirements.txt

# Install Rust components
RUN rustup component add rustfmt clippy

# Install Rust Nightly Build
RUN rustup install nightly

# Install Cargo Nextest
RUN cargo install cargo-nextest

# Install Cargo Bloat
RUN cargo install cargo-bloat

# Run Cargo build
CMD cargo build
