TARGET := aarch64-unknown-none-softfloat
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/mimicros
KERNEL_BIN := $(KERNEL_ELF).bin

PLATFORM ?= qemu_aarch64

ifeq ($(PLATFORM), qemu_aarch64)
	QEMU_MACHINE := virt,gic-version=3,virtualization=on
	QEMU_CPU := cortex-a76
	QEMU_CPUS := 4
	QEMU_MEM := 4G
else ifeq ($(PLATFORM), qemu_raspi3)
	QEMU_MACHINE := raspi3b
	QEMU_CPU := cortex-a72
	QEMU_CPUS := 4
	QEMU_MEM := 1G
endif

QEMU_OPTS := -machine $(QEMU_MACHINE) -cpu $(QEMU_CPU) -smp $(QEMU_CPUS) -m $(QEMU_MEM)
QEMU_OPTS += -serial stdio -display none
# QEMU_OPTS += -device loader,file=$(KERNEL_BIN),addr=0x3000000,force-raw=on
# QEMU_OPTS += -kernel u-boot-aarch64
QEMU_OPTS += -kernel $(KERNEL_BIN)

ifeq ($(MODE), release)
	MODE_ARG := --release
endif

export RUSTFLAGS=-Clink-arg=-Tsrc/platform/$(PLATFORM)/linker.ld

all: $(KERNEL_BIN)

clean:
	cargo clean
	rm -f .gdbinit

format:
	cargo fmt

kernel:
	cargo build $(MODE_ARG) --target aarch64-unknown-none-softfloat

$(KERNEL_BIN): kernel
	rust-objcopy $(KERNEL_ELF) --binary-architecture=aarch64 --strip-all -O binary $(KERNEL_BIN)

objdump:
	rust-objdump -S $(KERNEL_ELF) 2> /dev/null

run: $(KERNEL_BIN)
	qemu-system-aarch64 $(QEMU_OPTS)

test:
	cargo test

.gdbinit: .gdbinit.template
	sed "s:remote-target:$(KERNEL_ELF):" < $^ > $@

debug: .gdbinit kernel
	qemu-system-aarch64 $(QEMU_OPTS) -S -s

