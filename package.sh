#!/bin/bash

set -e

cargo +nightly build --release
cp target/release/test_game .
tar -caf merry_christmas.tar.gz test_game assets
rm test_game
