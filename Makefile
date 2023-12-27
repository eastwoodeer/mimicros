TARGET := aarch64-unknown-none
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/mimicros
KERNEL_BIN := $(KERNEL_ELF).bin

QEMU_MACHINE := virt,gic-version=3,virtualization=on
QEMU_CPU := cortex-a76
QEMU_CPUS := 4
QEMU_MEM := 1G

QEMU_OPTS := -machine $(QEMU_MACHINE) -cpu $(QEMU_CPU) -smp $(QEMU_CPUS) -m $(QEMU_MEM)
QEMU_OPTS += -serial stdio -display none
# QEMU_OPTS += -device loader,file=$(KERNEL_BIN),addr=0x3000000,force-raw=on
# QEMU_OPTS += -kernel u-boot-aarch64
QEMU_OPTS += -kernel $(KERNEL_ELF)

ifeq ($(MODE), release)
	MODE_ARG := --release
endif

all: $(KERNEL_BIN)

clean:
	cargo clean
	rm -f .gdbinit


kernel:
	cargo build $(MODE_ARG)

$(KERNEL_BIN): kernel
	rust-objcopy $(KERNEL_ELF) --binary-architecture=aarch64 --strip-all -O binary $(KERNEL_BIN)

objdump:
	rust-objdump -S $(KERNEL_ELF) 2> /dev/null

run: kernel
	qemu-system-aarch64 $(QEMU_OPTS)

.gdbinit: .gdbinit.template
	sed "s:remote-target:$(KERNEL_ELF):" < $^ > $@

debug: .gdbinit kernel
	qemu-system-aarch64 $(QEMU_OPTS) -S -s

