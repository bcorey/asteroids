#!/bin/sh
cargo build --target x86_64-pc-windows-gnu &&
cp ./target/x86_64-pc-windows-gnu/debug/zymartu-asteroids.exe . &&
exec ./zymartu-asteroids.exe "$@"