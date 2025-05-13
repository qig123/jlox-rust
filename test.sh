#!/bin/bash

set -euo pipefail

cargo build
cd ../craftinginterpreters
dart ./tool/bin/test.dart jlox-rust chap07_evaluating --interpreter ../jlox-rust/target/debug/jlox-rust