FROM ubuntu:latest as Builder
WORKDIR .
RUN apt update -y
RUN apt install git -y
RUN git clone https://github.com/tarunrawatknoldus/network_sniffer.git

FROM rust:latest
COPY --from=Builder ./network_sniffer/ ./network_sniffer/
WORKDIR ./network_sniffer/client/
EXPOSE 8000
CMD ["sudo","cargo","run"]