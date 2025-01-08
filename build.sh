#!/bin/bash

#simplified build script (follows rules in rustfmt.toml)
cargo fmt && cargo build --release