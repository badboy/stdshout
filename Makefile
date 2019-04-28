all: preload/target/release/stdshout_preload.so target/release/stdshout

target/release/stdshout: src/bin/stdshout.rs
	cargo build --release


preload/target/release/stdshout_preload.so: preload/src/lib.rs
	cd preload && cargo build --release


run: all
	@bash -c 'STDSHOUT_EXE=$$(readlink -f target/release/stdshout) LD_PRELOAD=./preload/target/release/stdshout_preload.so $${SHELL}'
