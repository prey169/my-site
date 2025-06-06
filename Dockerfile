# Builder stage
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

RUN apt update && apt install lld clang musl-tools -y

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
RUN rustup target add x86_64-unknown-linux-musl
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

RUN cargo build --release --bin my-site --target  x86_64-unknown-linux-musl
RUN strip target/x86_64-unknown-linux-musl/release/my-site


# Runtime stage
FROM scratch AS runtime

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/my-site /my-site
COPY --from=builder /app/configuration /configuration
COPY --from=builder /app/templates /templates

ENV APP_ENVIRONMENT=production
EXPOSE 8000

CMD ["/my-site"]
