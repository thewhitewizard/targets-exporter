FROM debian:stable-20241111-slim

RUN apt-get update && apt-get install -y ca-certificates curl
WORKDIR /app
COPY targets-exporter/target/aarch64-unknown-linux-gnu/release/targets-exporter /app/
COPY targets.txt /app/
EXPOSE 8080

CMD ["/app/targets-exporter"]