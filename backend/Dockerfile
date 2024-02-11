FROM rust:1.76 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --target=x86_64-unknown-linux-gnu

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install libsqlite3-0
LABEL description="Backend for the app that lets everyone know when the cats have been fed" \
      org.opencontainers.image.authors="filip@filipwieland.com"
COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/backend /app
ENTRYPOINT [ "/app" ]
