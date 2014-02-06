RUST_CRATE_FILENAME=rustc --crate-file-name

SAX=src/sax/lib.rs
SAX_CRATE=lib/$(shell $(RUST_CRATE_FILENAME) $(SAX))

all: bin/test

bin/.dummy:
	mkdir -p bin
	touch bin/.dummy

lib/.dummy:
	mkdir -p lib
	touch lib/.dummy

$(SAX_CRATE): lib/.dummy $(SAX)
	rustc --out-dir lib $(SAX)

bin/test: bin/.dummy $(SAX_CRATE) $(SAX)
	rustc --test -Llib $(SAX) -o bin/test

test: bin/test
	@bin/test

clean:
	rm -rf bin lib
