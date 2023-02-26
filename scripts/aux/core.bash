#!/usr/bin/env bash

export CHANNEL_HOME="$(cd "$(dirname "$0")/.." && pwd)"
export PROJ_HOME="$CHANNEL_HOME"
export PROJ_NAME="channel"
export CARGO_PATH="${CHANNEL_HOME}/core/Cargo.toml"

# TODO: bump dotfiles + remove this fn
log::note() { log::info "$@"; }
export -f log::note

dot::clone() {
  git clone 'https://github.com/denisidoro/dotfiles' "$DOTFILES"
  cd "$DOTFILES"
  git checkout 'v2022.07.16'
}

dot::clone_if_necessary() {
  [ -n "${DOTFILES:-}" ] && [ -x "${DOTFILES}/bin/dot" ] && return
  export DOTFILES="${CHANNEL_HOME}/target/dotfiles"
  dot::clone
}

dot::clone_if_necessary
