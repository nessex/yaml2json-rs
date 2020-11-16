#!/bin/bash

repo_root=$(git rev-parse --show-toplevel)
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
source "${DIR}/targets.sh"

targets=$(get_targets)

version=$(cargo workspaces list --json | jq -r '.[] | select(.name=="yaml2json-rs-bin") | .version')
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

    out="yaml2json-rs-v${version}-${target}.${extension}"

    atool -a "${out}" "${filename}"
    mv "${out}" "${version_dir}/${out}"
  )
}

while read -r target; do
  build_target "${target}"
done <<< "${targets}"
