#!/bin/sh

# Generate the peripheral access crate for the GD32F4 family of microcontrollers
svd2rust -i svd/gd32f4xx.svd

# Remove the old src directory and the old lib.rs file
# rm -rf src

# Generate the source files
# form -i lib.rs -o src/ && rm lib.rs

# Format the code
# cargo fmt
