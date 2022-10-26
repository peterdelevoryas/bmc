arch = arm-none-eabi
cpu = cortex-a7
mode = debug
dst = target/armv7a-none-eabi/$(mode)
elf = $(dst)/bmc
bin = $(dst)/bmc.bin
mtd = $(dst)/bmc.mtd

all: $(mtd) uart5.bin

uart5.bin: $(bin) gen_uart_booting_image.sh
	./gen_uart_booting_image.sh $(bin) $@

$(mtd): $(bin)
	dd if=/dev/zero of=$(mtd) bs=1M count=128
	dd if=$(bin) of=$(mtd) bs=1k conv=notrunc

$(bin): $(elf) 
	cargo objcopy -- -O binary $@

$(elf): src/main.rs src/start.S link.ld
	cargo build

.PHONY: objdump qemu

objdump: $(mtd)
	$(arch)-objdump -D -b binary -m $(cpu) $(mtd)

qemu: $(mtd)
	qemu-system-aarch64 \
	-machine ast2600-evb \
	-nographic \
	-drive file=$(mtd),format=raw,if=mtd,snapshot=on

gdb: $(elf)
	$(arch)-gdb $(elf) -ex "target remote localhost:1234"
