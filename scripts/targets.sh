#!/bin/bash

get_targets() {
  targets=$(cat <<-'EOF'
aarch64-unknown-linux-gnu
riscv64gc-unknown-linux-gnu
x86_64-pc-windows-gnu
x86_64-unknown-linux-gnu
EOF
)

  if [[ -z "${EXCLUDE_MACOS}" ]]; then
    printf "%s\n%s" "${targets}" "x86_64-apple-darwin"
  else
    echo "${targets}"
  fi
}
