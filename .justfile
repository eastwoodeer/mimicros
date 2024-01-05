TARGET := "aarch64-unknown-none"
MODE := "release"
TARGET_DIR := "target" / TARGET / MODE
KERNEL_ELF :=  TARGET_DIR / "mimicros"
KERNEL_BIN := KERNEL_ELF + ".bin"

alias r := run

default:
	make

run:
	make run

kernel:
	cargo build --release

kernel_bin: kernel
	rust-objcopy {{KERNEL_ELF}} --binary-architecture=aarch64 --strip-all -O binary {{KERNEL_BIN}}

dump:
	make objdump > {{KERNEL_ELF}}.asm
