#!/bin/bash

# Default to --speed if no argument is provided
if [ -z "$1" ] || [ "$1" == "--speed" ]; then
    cargo +nightly build -Z unstable-options -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu --profile=release-speed
elif [ "$1" == "--min" ]; then
    cargo +nightly build -Z unstable-options -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu --profile=release-min-size
else
    echo "Invalid argument. Please use --speed or --min."
fi
