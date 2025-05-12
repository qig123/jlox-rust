#!/bin/bash

set -euo pipefail

cargo build
cd ../craftinginterpreters
dart ./tool/bin/test.dart jlox-rust chap04_scanning --interpreter ../clox-rs/target/debug/jlox-rust