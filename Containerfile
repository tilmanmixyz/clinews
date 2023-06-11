FROM docker.io/rust:17.0-buster
WORKDIR /app
COPY . /app
RUN sudo apt install -y \
  libssl-dev pkg-config
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/clinews /
CMD ["./clinews"]