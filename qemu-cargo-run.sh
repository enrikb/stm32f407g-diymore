#! /bin/sh

export CARGO_TARGET_THUMBV7EM_NONE_EABIHF_RUNNER=\
"tools/qemu-runner.sh -machine netduinoplus2 -semihosting-config enable=on -nographic -kernel"

cargo run "$@"
