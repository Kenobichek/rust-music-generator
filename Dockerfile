FROM rust:1.91-slim

WORKDIR /app

COPY . .

RUN cargo install cargo-watch

ENTRYPOINT ["cargo", "watch"]
CMD ["--ignore", "target/*", "-x", "run", "-q"]
