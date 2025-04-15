FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
RUN cargo install cargo-watch
COPY --from=planner /app/recipe.json recipe.json
RUN rustup target add x86_64-unknown-linux-musl
# build dependencies - this is caching layer!!!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# build actual application
COPY . .
# crash if our application has logic errors, also cache 'check's for cargo
# watch to avoid running the run script on breaking changes
RUN cargo check --release --target x86_64-unknown-linux-musl --bin golf_server
RUN cargo build --release --target x86_64-unknown-linux-musl --bin golf_server
RUN strip /app/target/x86_64-unknown-linux-musl/release/golf_server

FROM builder AS develop
# use --poll to watch for saved changes on host volume end...
# use check to ensure we SHOULD be running our application, and it isnt just
# saved in an intermediate state...
CMD cargo watch -w src -w static -w templates \
  -x "check --release --target x86_64-unknown-linux-musl --bin golf_server" \
  --poll \
  -s "cargo run --release --target x86_64-unknown-linux-musl --bin golf_server"

FROM scratch AS runtime
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/golf_server ./golf_server
ENTRYPOINT ["./golf_server"]