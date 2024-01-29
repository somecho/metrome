#!/usr/bin/bash

for i in examples/valid/*; do
  cargo run -- -p $i
done
