all: build/rent-accounting

clean:
	rm -fR build

build/rent-accounting: build/guard
	rustc main.rs -o $@

build/guard:
	mkdir -p build
	touch $@