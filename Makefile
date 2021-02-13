.PHONY: all
all: build


.PHONY: build
build:
	cargo build

.PHONY: test
test:
	cargo test

.PHONY: clippy
clippy:
	cargo clippy --all-features

.PHONY: doc
doc:
	RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features --open

