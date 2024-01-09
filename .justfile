TARGET := "aarch64-unknown-none-softfloat"
MODE := "release"
TARGET_DIR := "target" / TARGET / MODE
KERNEL_ELF :=  TARGET_DIR / "mimicros"
KERNEL_BIN := KERNEL_ELF + ".bin"

alias r := run
alias f := fmt

default:
	make

run:
	make run

fmt:
	make format

kernel:
	cargo build --release

kernel_bin: kernel
	rust-objcopy {{KERNEL_ELF}} --binary-architecture=aarch64 --strip-all -O binary {{KERNEL_BIN}}

dump:
	make objdump > {{KERNEL_ELF}}.asm
