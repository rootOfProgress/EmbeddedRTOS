#!/bin/bash
gdb-multiarch -q ./target/thumbv7em-none-eabihf/debug/osmain \
-ex "target remote :3333" \
-ex "monitor reset halt" \
-ex "c" \
