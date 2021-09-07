#!/bin/bash

get_targets() {
  targets=$(cat <<-'EOF'
aarch64-unknown-linux-gnu
riscv64gc-unknown-linux-gnu
x86_64-pc-windows-gnu
x86_64-unknown-linux-musl
EOF
)

  macos_targets=$(cat <<-'EOF'
aarch64-apple-darwin
x86_64-apple-darwin
EOF
)

  if [[ -n "${ONLY_MACOS}" ]]; then
    echo "${macos_targets}"
  elif [[ -z "${EXCLUDE_MACOS}" ]]; then
    printf "%s\n%s" "${targets}" "${macos_targets}"
  else
    echo "${targets}"
  fi
}
