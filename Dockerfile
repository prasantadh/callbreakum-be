FROM rust:1.67 as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo install --path api/

FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/api /usr/local/bin/api
CMD ["api"]

