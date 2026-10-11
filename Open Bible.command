#!/bin/zsh
cd -- "${0:A:h}" || exit 1
exec ./target/release/bible-rust
