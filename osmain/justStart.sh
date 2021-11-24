#!/bin/bash
# this script helps to run or restart the os without
# rendering down the flash memory by writing the same stuff
# on it every time 
gdb-multiarch -q ./target/thumbv7em-none-eabihf/debug/osmain \
-ex "target remote :3333" \
-ex "monitor reset halt" \
-ex "c" \
