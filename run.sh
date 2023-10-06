#!/bin/bash
export HERMES_KEY_PATH="${PWD}/cert/localhost.decrypted.key"
export HERMES_CERT_PATH="${PWD}/cert/localhost.crt"
export HERMES_ROOT_PATH="${PWD}"
export PORT=8080

RUST_LOG="debug,poem=debug" cargo run