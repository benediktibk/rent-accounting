all: build/rent-accounting

clean:
	rm -fR build
	rm -fR rent-accounting/target

build/rent-accounting: build/guard
	cargo build --manifest-path rent-accounting/Cargo.toml
	cp rent-accounting/target/debug/rent-accounting $@

build/guard: Makefile
	mkdir -p build
	touch $@