# Build Stage
FROM rust:latest as Builder
WORKDIR /app
RUN apt-get update && \
    apt-get install -y git && \
    git clone https://github.com/tarunrawatknoldus/network_sniffer.git

WORKDIR /app/network_sniffer/actix/
RUN cargo build --release

# Final Stage
FROM rust:latest
COPY --from=Builder /app/network_sniffer/actix/target/release/actix_app /app/
WORKDIR /app
EXPOSE 2000
CMD ["sudo", "cargo", "run"]
