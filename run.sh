#!/bin/bash

for pid in $(lsof -i :8080 | grep LISTEN | awk '{print $2}'); do
  kill -9 $pid
done
for pid in $(lsof -i :8081 | grep LISTEN | awk '{print $2}'); do
  kill -9 $pid
done

cargo build --target wasm32-unknown-unknown --features with-wasm
wasm-pack build --target web --features with-wasm
serve -s . -l 8080 --cors &
cargo run --release --features no-wasm
wait

