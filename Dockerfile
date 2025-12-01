FROM rust:1.91-slim

RUN apt-get update && apt-get install -y \
    curl build-essential bash \
    && rm -rf /var/lib/apt/lists/*

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

RUN cargo install cargo-watch --quiet || true

WORKDIR /app

COPY . .

RUN wasm-pack build --target web --out-dir ./web/pkg --release

ENTRYPOINT ["cargo", "watch", "-i", "./web/pkg/*", "-i", "target/*", "-s", "wasm-pack build --target web --out-dir ./web/pkg --release --quiet"]
