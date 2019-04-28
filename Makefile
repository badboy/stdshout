all: preload/target/release/libstdshout_preload.so target/release/stdshout

target/release/stdshout: src/bin/stdshout.rs
	cargo build --release


preload/target/release/libstdshout_preload.so: preload/src/lib.rs
	cd preload && cargo build --release


run: all
	@bash -c 'STDSHOUT_EXE=$$(readlink -f target/release/stdshout) LD_PRELOAD=./preload/target/release/libstdshout_preload.so $${SHELL}'
