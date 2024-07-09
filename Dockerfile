FROM rust:1.79 as builder

RUN cargo install sqlx-cli --no-default-features --features sqlite

FROM builder AS build

WORKDIR /usr/src/mock
COPY . .

RUN sqlx database setup
RUN cargo build --release

FROM rust:1.79

COPY --from=build /usr/src/mock/target/release/mock /usr/local/bin/mock

ARG PORT=8888
ENV MOCK_ADDRESS=0.0.0.0 MOCK_PORT=$PORT MOCK_TOKEN='token'
EXPOSE $PORT/tcp

CMD ["mock"]