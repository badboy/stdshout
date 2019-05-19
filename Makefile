all: target/release/libstdshout_preload.so target/release/stdshout

target/release/stdshout: stdshout/src/bin/stdshout.rs stdshout/src/lib.rs
	cargo build --release

target/release/libstdshout_preload.so: preload/src/lib.rs
	cargo build --release


run: all
	@bash -c 'STDSHOUT_EXE=$$(readlink -f target/release/stdshout) LD_PRELOAD=./target/release/libstdshout_preload.so sh'
