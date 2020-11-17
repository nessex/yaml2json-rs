FROM ekidd/rust-musl-builder AS builder
# Add our source code.
ADD . ./
# build for musl
RUN cargo build --release


FROM gcr.io/distroless/base:nonroot@sha256:2b177fbc9a31b85254d264e1fc9a65accc6636d6f1033631b9b086ee589d1fe2
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/yaml2json /yaml2json
EXPOSE 9090
ENTRYPOINT ["/yaml2json"]
