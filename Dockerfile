FROM rust:latest AS build
ADD . /app
WORKDIR /app
RUN cargo build --release
FROM debian:buster-slim
COPY --from=build /app/target/release/diskonaut /bin/diskonaut
ENTRYPOINT ["/bin/diskonaut"]