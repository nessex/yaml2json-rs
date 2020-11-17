FROM ekidd/rust-musl-builder AS builder
ADD . ./
RUN cargo build --release


FROM gcr.io/distroless/base:nonroot@sha256:0b257d9ec4d1f7ba99578e6a4dd516473afe00fde934506c5b62b606b814aa2e
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/yaml2json /yaml2json
ENTRYPOINT ["/yaml2json"]
