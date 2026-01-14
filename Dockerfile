FROM rust:1.85

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build

EXPOSE 8000
CMD ["cargo", "run"]
