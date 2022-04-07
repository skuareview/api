FROM rustlang/rust:nightly

WORKDIR /app

ADD src src
COPY Cargo.toml .

RUN cargo install cargo-watch
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo build
