QEMU = qemu-system-riscv32
QFLAGS = -nographic -smp 1 -machine virt -bios none

CROSS_COMPILE = riscv64-unknown-elf-
CFLAGS = -nostdlib -fno-builtin -march=rv32ima -mabi=ilp32 -g -Wall
GDB = ${CROSS_COMPILE}gdb
CC = ${CROSS_COMPILE}gcc

TARGET      := riscv32imac-unknown-none-elf
MODE        := debug
KERNEL_FILE := target/$(TARGET)/$(MODE)/os
BIN_FILE    := target/$(TARGET)/$(MODE)/kernel.bin

OBJDUMP     := rust-objdump --arch-name=riscv32
OBJCOPY     := rust-objcopy --binary-architecture=riscv32

all: $(BIN_FILE)

# 编译 kernel
kernel:
	@cargo build

$(BIN_FILE): kernel
	@$(OBJCOPY) $(KERNEL_FILE) -O binary $@

# 通过 Rust 文件中的注释生成文档
.PHONY: doc
doc:
	@cargo doc --document-private-items

.PHONY: asm
asm:
	@$(OBJDUMP) -d $(KERNEL_FILE) | less

# 清理编译出的文件
.PHONY: clean
clean:
	@cargo clean

run: all
	@${QEMU} -M ? | grep virt >/dev/null || exit
	@echo "Press Ctrl-A and then X to exit QEMU"
	@echo "------------------------------------"
	@${QEMU} ${QFLAGS} -kernel $(BIN_FILE)

.PHONY: debug
debug: all
	@echo "Press Ctrl-C and then input 'quit' to exit GDB and QEMU"
	@echo "-------------------------------------------------------"
	@${QEMU} ${QFLAGS} -kernel $(BIN_FILE) -s -S &
	@${GDB} $(KERNEL_FILE) -q -x ./gdbinit