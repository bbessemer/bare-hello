CFLAGS += -O2 -ffreestanding -mno-red-zone -mno-sse -nostdlib

all: kernel.bin

kernel.bin: kernel.o
	ld -m elf_i386 --oformat=binary -T link.ld $^ -o $@

%.o: %.c
	clang --target=i386-none-none -c $< -o $@

clean:
	@rm -rvf *.o *.elf
