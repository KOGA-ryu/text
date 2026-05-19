#!/usr/bin/env bash
set -euo pipefail

expected_url="https://github.com/KOGA-ryu/text.git"

if git remote get-url origin >/dev/null 2>&1; then
  current_url="$(git remote get-url origin)"
  if [[ "${current_url}" != "${expected_url}" ]]; then
    echo "Updating origin from ${current_url} to ${expected_url}"
    git remote set-url origin "${expected_url}"
  fi
else
  echo "Adding origin ${expected_url}"
  git remote add origin "${expected_url}"
fi

git remote -v

dirty_paths="$(git status --short)"
if [[ -n "${dirty_paths}" ]]; then
  echo "Workspace is dirty after remote setup:"
  echo "${dirty_paths}"
  exit 1
fi

echo "Cloud remote setup is ready."
