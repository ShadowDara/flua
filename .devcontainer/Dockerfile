# Start from the official Rust Dev Container base image
FROM mcr.microsoft.com/devcontainers/rust:1

# Set working directory
WORKDIR /workspace

# Make sure cargo installs and caches happen in the user directory
ENV CARGO_HOME=/usr/local/cargo

# Install system dependencies
RUN apt-get update && \
    apt-get install -y \
        build-essential \
        pkg-config \
        libssl-dev \
        python3 \
        python3-pip \
        python3-venv \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Create venv directory and adjust permissions
RUN mkdir -p /opt/venv && chown -R vscode:vscode /opt/venv

# Set up Python virtual environment
RUN python3 -m venv /opt/venv
ENV PATH="/opt/venv/bin:$PATH"

# Copy requirements and install
COPY build/requirements.txt build/requirements.txt
RUN pip install --no-cache-dir -r build/requirements.txt

# Install Rust components
RUN rustup component add rustfmt clippy
RUN rustup install nightly

# Copy project files into container
COPY . .

# Make sure workspace is writable by the dev user
RUN chown -R vscode:vscode /workspace

# Switch to the non-root user for running commands
USER vscode

# sudo chown -R $(whoami):$(whoami) /workspace
