#!/bin/bash

repo_root=$(git rev-parse --show-toplevel)
source "${repo_root}/scripts/targets.sh"

targets=$(get_targets)

version=$(grep --max-count=1 'version\ ' < 'crates/yaml2json-rs-bin/Cargo.toml' | cut -d' ' -f 3 | tr -d '"')
version_dir="${repo_root}/target/versions/${version}"

mkdir -p "${version_dir}"

build_target() {
  local target
  target="${1}"

  cross build --target "${target}" --release

  # use a sub-shell so we don't have to worry about the cd, or if pushd / popd are available
  (
    local extension filename

    cd "${repo_root}/target/${target}/release" || exit

    case "${target}" in
      x86_64-pc-windows-gnu)
        extension="zip"
        filename="yaml2json.exe"
        ;;
      *)
        # By default, create a .tar.gz
        extension="tar.gz"
        filename="yaml2json"
        ;;
    esac

    if command -v strip 1>/dev/null; then
      strip "${filename}"
    fi

    out="yaml2json-rs-v${version}-${target}.${extension}"

    atool -qa "${out}" "${filename}"
    mv "${out}" "${version_dir}/${out}"
  )
}

while read -r target; do
  build_target "${target}"
done <<< "${targets}"
