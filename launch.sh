#!/bin/bash

# First run this to flash the bootloader and partition table
# cargo run --release

espflash save-image --chip esp32c6 target/riscv32imac-unknown-none-elf/release/probe-rs-issue target/riscv32imac-unknown-none-elf/release/probe-rs-issue-small
esptool.py write_flash 0x00010000 target/riscv32imac-unknown-none-elf/release/probe-rs-issue-small
sleep 2
probe-rs attach target/riscv32imac-unknown-none-elf/release/probe-rs-issue