.PHONY: all
all: build


.PHONY: build
build:
	cargo build --all-features

.PHONY: test
test:
	cargo test --all-features

.PHONY: clippy
clippy:
	cargo clippy --all-features

.PHONY: doc
doc:
	RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features --no-deps --open

