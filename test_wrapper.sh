#!/bin/zsh
# Test wrapper
echo "Wrapper called with args: $@"
noglob ./target/debug/livemd "$@"
