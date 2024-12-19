#!/bin/bash

set -e
set -x

d="$(date -u '+%Y.%m.%d')"

n="$(cat Cargo.toml | grep -n -F '##BUMP-VERSION.SH##' | tr ':' ' ' | awk '{print $1}')"
l="$(cat Cargo.toml | wc -l)"

{
    cat Cargo.toml | head -n $[n-1]
    echo "version = \"${d}\" ##BUMP-VERSION.SH##"
    cat Cargo.toml | tail -n $[l-n]
} > .Cargo.toml.new

mv .Cargo.toml.new Cargo.toml
