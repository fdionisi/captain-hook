#!/bin/sh

# Adapted from `husky.sh`
# @see https://github.com/typicode/husky/blob/a8e427a3db58086afd3ebe5d9595e30c6837b216/husky.sh

if [ -z "$captain_hook_skip_init" ]; then
  debug () {
    [ "$CAPTAIN_HOOK_DEBUG" = "1" ] && echo "captain-hook (debug) - $1"
  }

  readonly hook_name="$(basename "$0")"
  debug "starting $hook_name..."

  if [ "$CAPTAIN_HOOK" = "0" ]; then
    debug "CAPTAIN_HOOK env variable is set to 0, skipping hook"
    exit 0
  fi

  if [ -f ~/.captainhookrc ]; then
    debug "sourcing ~/.captainhookrc"
    . ~/.captainhookrc
  fi

  export readonly captain_hook_skip_init=1
  sh -e "$0" "$@"
  exit_code="$?"

  if [ $exit_code != 0 ]; then
    echo "captain-hook - $hook_name hook exited with code $exit_code (error)"
  fi

  exit $exit_code
fi