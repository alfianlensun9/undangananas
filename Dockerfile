FROM rust:1.80.1 AS build
WORKDIR /app
ENV TZ="Asia/Makassar"
COPY . .
RUN cargo build --release


FROM debian:bookworm-slim AS runtime
WORKDIR /app
ENV TZ="Asia/Makassar"

COPY --from=build /app/target/release/undangan .
COPY --from=build /app/.env .env
EXPOSE 8080

CMD ["/app/undangan"]