#!/usr/bin/bash

for i in $(pwd)/examples/valid/*; do
  cargo run -- -p $i -o $(pwd)/tests/assets/wav/$(basename $i).wav
done
