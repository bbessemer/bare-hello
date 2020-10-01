all: kernel.bin

kernel.bin: bare-hello
	objcopy $< -O binary $@

bare-hello:
	cargo build --release --out-dir . -Z unstable-options

clean:
	rm -rvf target/ bare-hello

.PHONY: bare-hello clean
