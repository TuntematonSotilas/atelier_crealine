# Get started with a build env with Rust nightly
FROM rust:1.95.0-alpine3.22 AS builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen perl make

RUN npm install -g sass
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work

# Copy only manifest files first to cache dependency resolution and compilation
COPY Cargo.toml ./
RUN cargo install --locked cargo-leptos
RUN cargo fetch --locked

COPY package.json package-lock.json ./
RUN npm install

# Copy the rest of the sources and build
COPY . .
RUN cargo leptos build --release

FROM rust:1.95.0-alpine3.22 AS runner

WORKDIR /app

COPY --from=builder /work/target/release/atelier_crealine /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT=./site
EXPOSE 8080

CMD ["/app/atelier_crealine"]
