FROM rust:1.73 as builder

WORKDIR /usr/src/website
COPY . .

RUN apt-get install pkg-config

RUN cargo install --path .

FROM debian as final



RUN apt-get update && apt-get install -y libssl3

WORKDIR /usr/local/bin/

COPY ./src/ai/dataset.csv ./src/ai/dataset.csv

COPY ./src/pages ./src/pages

COPY --from=builder /usr/local/cargo/bin/ .
CMD ["website"]