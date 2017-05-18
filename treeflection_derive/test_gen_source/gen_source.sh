#!/bin/sh
cargo rustc --test test -- -Zunstable-options --pretty=expanded > gen_source.rs
