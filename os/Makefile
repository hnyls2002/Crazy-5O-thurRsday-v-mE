# Logger level: Trace, Debug, Info, Warn, Error
LOG ?= Trace

# Utils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin
MODE_ARG := --$(MODE)

# Bootloader
BOOTLOADER := none

# KERNEL ENTRY
KERNEL_ENTRY_PA := 0x80000000

#Disassembly
DISASM ?= -d -S
DISASM_TMP := target/$(TARGET)/$(MODE)/asm

clean:
	@cargo clean

build: $(KERNEL_BIN)

$(KERNEL_BIN): kernel
	@$(OBJCOPY) --strip-all $(KERNEL_ELF) -O binary $(KERNEL_BIN)

kernel:
	@cd ../user && make build
	@cargo build $(MODE_ARG) --features $(LOG)
	@file $(KERNEL_ELF)

disasm: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) > $(DISASM_TMP)
	@vim $(DISASM_TMP)
	@rm $(DISASM_TMP)


run: build
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

gdbserver: build
	@qemu-system-riscv64 -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) -s -S

gdbclient:
	@riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'

.PHONY: build run gdbserver gdbclient