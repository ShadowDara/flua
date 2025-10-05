# Start from the official Rust Dev Container base image
FROM mcr.microsoft.com/devcontainers/rust:1

# Set working directory (optional – VS Code setzt das eh korrekt)
WORKDIR /workspace

# Copy only requirements file to install Python dependencies
COPY requirements.txt .

# Install pip
RUN apt-get update && apt-get install -y python3 python3-pip

# Install Python packages
RUN pip install -r requirements.txt
