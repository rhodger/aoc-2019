FROM rust:1.39 as builder
RUN mkdir /app
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update
COPY --from=builder /app/target/release/aoc-2019 /usr/local/bin/
CMD ["aoc-2019", "-h"]
