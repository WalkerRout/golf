FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

ARG RUST_TARGET
ENV RUST_TARGET=$RUST_TARGET

# separate stage for esbuild binary
FROM alpine:3.19 AS esbuild
RUN apk add --no-cache curl \
  && curl -fsSL https://registry.npmjs.org/@esbuild/linux-x64/-/linux-x64-0.19.11.tgz | tar -xz \
  && mv package/bin/esbuild /esbuild \
  && chmod +x /esbuild

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# need musl to run with glibc statically linked on scratch image...
RUN apt-get update && apt-get install -y \
  musl-tools \
  pkg-config \
  libssl-dev \
  && rm -rf /var/lib/apt/lists/*
# copy esbuild binary for typescript compilation in build.rs
COPY --from=esbuild /esbuild /usr/local/bin/esbuild
RUN rustup target add $RUST_TARGET
# build dependencies - this is caching layer!!!
RUN cargo chef cook --release --target $RUST_TARGET --recipe-path recipe.json
# build actual application
COPY . .
RUN cargo build --release --target $RUST_TARGET --bin golf_server
RUN strip /app/target/$RUST_TARGET/release/golf_server

FROM builder AS development
# crash if our application has logic errors, also cache 'check's for cargo
# watch to avoid running the run script on breaking changes
RUN cargo check --release --target $RUST_TARGET --bin golf_server
# install hot reload helper
RUN cargo install cargo-watch
# use --poll to watch for saved changes on host volume end...
# use check to ensure we SHOULD be running our application, and it isnt just
# saved in an intermediate state...
# we watch for frontend changes too, since the backend/build.rs file will
# recompile the frontend typescript into javascript before compressing /static
CMD cargo watch -w backend -w config -w frontend \
  -i "backend/static/**/*.gz" -i "backend/static/**/*.br" \
  -i "backend/static/**/*.js" -i "backend/static/**/*.js.map" \
  -x "check --release --target $RUST_TARGET --bin golf_server" \
  --poll \
  -s "cargo run --release --target $RUST_TARGET --bin golf_server"

FROM scratch AS production
# renew args
ARG RUST_TARGET
ENV RUST_TARGET=$RUST_TARGET
WORKDIR /app
COPY --from=builder /app/backend/static/ ./backend/static/
COPY --from=builder /app/target/$RUST_TARGET/release/golf_server ./backend/golf_server
ENTRYPOINT ["./backend/golf_server"]
