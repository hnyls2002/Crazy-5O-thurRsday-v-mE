build: 
	@cargo build --release
	@file target/riscv64gc-unknown-none-elf/release/os
	@rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin

run: build
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios none \
		-device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80000000

run_with_gdb: build
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios none \
		-device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80000000 \
		-s -S

gdb: 
	riscv64-unknown-elf-gdb \
    	-ex 'file target/riscv64gc-unknown-none-elf/release/os' \
    	-ex 'set arch riscv:rv64' \
    	-ex 'target remote localhost:1234'