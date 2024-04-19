FROM rust:1 as cargo-build

COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/static-debian12:latest

COPY --from=cargo-build /root/.cargo/bin/cedar-server /usr/local/bin/cedar-server

ENV RUST_LOG=info

CMD ["cedar-server"]