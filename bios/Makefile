all: kernel.bin

kernel.bin: kernel.asm
	nasm -f bin $< -o $@

clean:
	@rm -rvf kernel.bin
