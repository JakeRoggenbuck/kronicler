all:
	maturin build
	pip install --force-reinstall target/wheels/logfrog-*

test: all
	RUST_LOG=info python3 simple_test.py
