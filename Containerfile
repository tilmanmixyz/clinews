FROM docker.io/rust:slim-bookworm AS build
WORKDIR /app
COPY . /app
RUN apt-get install -y \
  libssl-dev pkg-config
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build /app/target/release/clinews /
CMD ["./clinews"]
