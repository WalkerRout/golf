FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# build dependencies - this is caching layer!!!
RUN cargo chef cook --release --recipe-path recipe.json
# build actual application
COPY . .
RUN cargo build --release --bin golf_server

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/golf_server ./golf_server
COPY --from=builder /app/templates/ ./templates/
COPY --from=builder /app/static/ ./static/
ENTRYPOINT ["./golf_server"]