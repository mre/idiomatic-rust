FROM rust as builder

ADD . /app
WORKDIR /app
RUN cargo build --release

FROM rust:1-slim
LABEL maintainer="Matthias Endler <matthias-endler@gmx.net>"

WORKDIR /app
COPY --from=builder /app/target/release/ci /app/ci
ENTRYPOINT [ "./ci" ]