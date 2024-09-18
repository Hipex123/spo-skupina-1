FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo install dioxus-cli

RUN dx build

EXPOSE 3104

CMD ["dx","serve","-p","3104"]
