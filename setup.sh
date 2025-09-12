#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: ./setup.sh <number>(01, 02...)"
  exit 1
fi

git switch main &&
git pull &&
git switch -c knock${1} &&
cargo new knock${1} &&
cargo check &&
git add . &&
git commit -m "feat: add knock${1}"