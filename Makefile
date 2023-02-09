.PHONY: docs docs-open docs-private

docs:
	RUSTDOCFLAGS="--html-in-header katex-header.html" cargo +nightly doc --no-deps

docs-open:
	RUSTDOCFLAGS="--html-in-header katex-header.html" cargo +nightly doc --no-deps --open

docs-private:
	RUSTDOCFLAGS="--html-in-header katex-header.html" cargo +nightly doc --no-deps --document-private-items
