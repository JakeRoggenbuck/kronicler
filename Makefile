all:
	maturin build
	pip install --force-reinstall target/wheels/logfrog-*
