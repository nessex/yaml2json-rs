#!/bin/bash

# Output is silent, unless there are errors

repo_root=$(git rev-parse --show-toplevel)
source "${repo_root}/scripts/targets.sh"

targets=$(get_targets)

while read -r target; do
  errors=$(
    cross test --message-format json --target "${target}" --quiet 2>/dev/null |
      # We only need the result rows which also state the test name
      grep ' \.\.\. ' |
      grep -v ' \.\.\. ok$'
  )

  if [[ -n "${errors}" ]]; then
    echo "[ERR(${target})]: "
    echo "${errors}"
    exit 1
  fi
done <<< "${targets}"

exit 0
