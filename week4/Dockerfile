<<<<<<< HEAD
FROM rust:1.75 AS builder

# Set the working directory
WORKDIR /usr/src/week4_us26

# Change the user to root
USER root

# Copy everything
COPY . .

# Build the project
RUN cargo build --release

# Expose port 8080
EXPOSE 8080

# Define the default command to run
CMD cargo run
=======
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
>>>>>>> 6f0addf99348ad839492e30bb882f7f81d14f5dc
