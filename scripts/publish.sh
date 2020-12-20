#!/bin/bash

repo_root=$(git rev-parse --show-toplevel)
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

# The order here matters, as yaml2json-rs-bin depends upon yaml-split and yaml2json-rs
cargo publish --manifest-path "${repo_root}/crates/yaml-split/Cargo.toml"
cargo publish --manifest-path "${repo_root}/crates/yaml2json-rs/Cargo.toml"

# Hack: wait for the other crates' latest versions to become visible
sleep 5
cargo publish --manifest-path "${repo_root}/crates/yaml2json-rs-bin/Cargo.toml"

