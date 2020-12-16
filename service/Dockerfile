# Build the code first
FROM rust:slim-buster AS builder

WORKDIR /universe

# First install any needed system dependencies
RUN rustc --version --verbose
RUN apt update -y && apt install -y pkg-config libssl-dev libclang-dev clang

# Download and compile the rust dependencies
RUN USER=universe cargo init --bin --name universe .
COPY Cargo.lock Cargo.toml /universe/

RUN USER=universe cargo init --lib --name=universe-testdatabase crates/testdatabase
COPY crates/testdatabase/Cargo.lock crates/testdatabase/Cargo.toml /universe/crates/testdatabase/

RUN cargo build --release

# Then we trash our actual build so that we can correctly build again
RUN find /universe/target -name "*universe*" | xargs rm -rf

# Now build the actual app
RUN rm -rf /universe/src
COPY src /universe/src

RUN rm -rf /universe/crates/testdatabase/src
COPY crates/testdatabase/src /universe/crates/testdatabase/src

COPY migrations /universe/migrations

RUN cargo build --release

# Next build a container with the build artifact but no code
FROM debian:buster-slim

RUN apt update -y && apt install -y openssl ca-certificates wget

ENV DOCKERIZE_VERSION v0.6.1
RUN wget https://github.com/jwilder/dockerize/releases/download/$DOCKERIZE_VERSION/dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
  && tar -C /usr/local/bin -xzvf dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
  && rm dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz

WORKDIR /universe

COPY --from=builder /universe/target/release/universe /universe/universe
COPY docker/start.sh /universe

ENV PORT=8000
ENV RUST_LOG=debug

CMD ["/universe/start.sh"]