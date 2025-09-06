all:
	maturin build
	pip install --force-reinstall target/wheels/kronicler-*manylinux*

test: all
	RUST_LOG=info python3 tests/simple_test.py
