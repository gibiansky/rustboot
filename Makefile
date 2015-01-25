LD=i386-elf-ld
RUSTC=/Users/silver/dev/rust/bin/rustc
NASM=nasm
QEMU=qemu-system-i386

all: floppy.img

.SUFFIXES: .o .rs .asm

.PHONY: clean run

.rs.o:
	$(RUSTC) -O -C no-stack-check --target i686-unknown-linux-gnu --extern core=/Users/silver/code/rustboot/core/target/i686-unknown-linux-gnu/libcore-7d9212b05ce37abd.rlib --crate-type lib -C relocation-model=static -o $@ --emit obj $<

.asm.o:
	$(NASM) -f elf32 -o $@ $<

floppy.img: loader.bin main.bin
	dd if=/dev/zero of=$@ bs=512 count=2 &>/dev/null
	cat $^ | dd if=/dev/stdin of=$@ conv=notrunc &>/dev/null

loader.bin: loader.asm
	$(NASM) -o $@ -f bin $<

main.bin: linker.ld main.o
	$(LD) -m elf_i386 -o $@ -T $^  /Users/silver/code/rustboot/core/target/i686-unknown-linux-gnu/libcore-7d9212b05ce37abd.rlib 

run: floppy.img
	$(QEMU) -fda $<

clean:
	rm -f *.bin *.o *.img
