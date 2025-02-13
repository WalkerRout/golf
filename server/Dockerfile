FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN rustup target add x86_64-unknown-linux-musl
# build dependencies - this is caching layer!!!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# build actual application
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin golf_server
RUN strip /app/target/x86_64-unknown-linux-musl/release/golf_server

FROM scratch AS runtime
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/golf_server ./golf_server
ENTRYPOINT ["./golf_server"]