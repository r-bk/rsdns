.PHONY: all
all: build


.PHONY: build
build:
	cargo build

.PHONY: clippy
clippy:
	cargo clippy --all-features

.PHONY:
doc:
	RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features --open

