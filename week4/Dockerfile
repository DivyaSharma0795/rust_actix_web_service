FROM rust:1.57 AS builder

# Set the working directory
WORKDIR /usr/src/week4

# Change the user to root
USER root

# Copy everything
COPY . .

# Build the project
# RUN cargo run

# Expose port 8080
EXPOSE 8080

# Define the default command to run
CMD cargo run