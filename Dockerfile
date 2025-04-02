# Run with eg.
# docker build -t gameboy-3d .
# docker create --name temp-container gameboy-3d
# docker cp temp-container:/app/blobgoes3d .
# docker rm temp-container

FROM rust:latest AS builder

WORKDIR /app

RUN cargo install agb-gbafix

COPY . .

RUN cargo build --release 

RUN agb-gbafix /app/target/thumbv4t-none-eabi/release/blobgoes3d -o blobgoes3d.gba
