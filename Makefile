install:
	cargo build
	mkdir -p ~/.cdd/
	mkdir -p ~/.cdd/bin
	cp -f target/debug/cdd ~/.cdd/bin/cdd
