#!/bin/bash

# Output is silent, unless there are errors

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
source "${DIR}/targets.sh"

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
