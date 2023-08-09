# Builder Stage
FROM ubuntu:latest as Builder
WORKDIR /
RUN apt update && \
    apt install -y git && \
    git clone https://github.com/tarunrawatknoldus/network_sniffer.git

# Rust Build Stage
FROM rust:latest as RustBuilder
COPY --from=Builder /network_sniffer/ /network_sniffer/
WORKDIR /network_sniffer/client/
RUN cargo build --release

# Final Stage
FROM rust:latest
COPY --from=RustBuilder /network_sniffer/client/target/release/ /app/
WORKDIR /app
EXPOSE 8000
CMD ["sudo", "cargo", "run"]
