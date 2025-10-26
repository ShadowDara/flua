# Start from the official Rust Dev Container base image
FROM mcr.microsoft.com/devcontainers/rust:1

# Set working directory
WORKDIR /workspace

# Make sure cargo installs and caches happen in the user directory
ENV CARGO_HOME=/usr/local/cargo
# Alternatively, if you prefer per-user instead of global installs:
# ENV CARGO_HOME=/home/vscode/.cargo
# ENV PATH="${CARGO_HOME}/bin:${PATH}"

# Install system dependencies
USER root
RUN apt-get update && \
    apt-get install -y \
        build-essential \
        pkg-config \
        libssl-dev \
        python3 \
        python3-pip \
        python3-venv \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Switch back to vscode user
USER vscode

# Set up Python virtual environment
RUN python3 -m venv /opt/venv
ENV PATH="/opt/venv/bin:$PATH"
COPY build/requirements.txt build/requirements.txt
RUN pip install --no-cache-dir -r build/requirements.txt

# Install Rust components
RUN rustup component add rustfmt clippy

# Install Rust nightly
RUN rustup install nightly

# Install useful Cargo tools (in user space)
RUN cargo install --locked cargo-nextest \
 && cargo install --locked cargo-bloat

# Copy project files into container
COPY . .

# Default command
CMD ["cargo", "build"]
