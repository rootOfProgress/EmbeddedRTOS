#!/bin/bash
rm -rfv ./target
cargo b
arm-none-eabi-gdb -q ./target/thumbv7em-none-eabihf/debug/osmain -ex "target remote :3333" -ex "monitor reset halt" -ex "load" -ex "monitor reset halt" -ex "si"