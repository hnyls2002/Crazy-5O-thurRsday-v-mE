TARGET := riscv64gc-unknown-none-elf
MODE := release
APP_DIR := src/bin
TARGET_DIR := target/$(TARGET)/$(MODE)
APPS := $(wildcard $(APP_DIR)/*.rs)
ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))
BINS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%.bin, $(APPS))

OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

LOG ?= Trace

elf: $(APPS)
	@cargo build --features $(LOG) --release

binary: elf
	@$(foreach elf, $(ELFS), $(OBJCOPY) $(elf) --strip-all -O binary $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.bin, $(elf));)

build: binary

app ?= hello

disasm : elf
	@$(OBJDUMP) -d $(TARGET_DIR)/$(app) > $(TARGET_DIR)/$(app).asm
	@vim $(TARGET_DIR)/$(app).asm
	@rm $(TARGET_DIR)/$(app).asm


clean:
	@cargo clean

.PHONY: elf binary build clean