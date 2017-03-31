all: stdshout_preload.so target/release/stdshout

target/release/stdshout: src/bin/stdshout.rs
	cargo build --release


stdshout_preload.so: stdshout_preload.c
	gcc -O2 -o $@ $< -fpic -shared -ldl -lc


run: all
	@bash -c 'STDSHOUT_EXE=$$(readlink -f target/release/stdshout) LD_PRELOAD=./stdshout_preload.so $${SHELL}'
