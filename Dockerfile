# syntax=docker/dockerfile:1
FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo install dioxus-cli

EXPOSE 3104

CMD ["dx","serve","--port","3104"]
