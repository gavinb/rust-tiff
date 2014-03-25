
RUSTC=rustc
RUST_FLAGS=-g

SOURCES=tiff.rs

%:%.rs
	$(RUSTC) $(RUST_FLAGS) $<

all:		tiff eg

tiff:		src/lib.rs src/reader.rs src/writer.rs
		rustc src/lib.rs

eg:		examples/test1.rs
		rustc -L . examples/test1.rs

clean:		#
		@rm -f libtiff*.dylib libtiff*.rlib test1
