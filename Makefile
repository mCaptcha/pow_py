define test_pow_sha_256_py ## Test test_pow_sha_256_py library
	echo "[*] testing libgit"
	cargo test --all --all-features --no-fail-fast
endef

default: ## Run app
	. ./venv/bin/activate && maturin build
	. ./venv/bin/activate && python -m interface

#coverage:
#	# rustup component add llvm-tools-preview is required
#	$(call test_interface)
#	@. ./venv/bin/activate && ./scripts/coverage.sh --coverage
#	@. ./venv/bin/activate && coverage xml && coverage html
#	@. ./venv/bin/activate && ./scripts/coverage.sh --coverage
#

env: ## Install all dependencies
	@-virtualenv venv
	. ./venv/bin/activate && pip install maturin
	. ./venv/bin/activate && maturin develop
	. ./venv/bin/activate && pip install -r requirements.txt
#	. ./venv/bin/activate && pip install -e .
	#. ./venv/bin/activate && pip install '.[test]'

freeze: ## Freeze python dependencies
	@. ./venv/bin/activate && pip freeze > requirements.txt
	@-sed -i '/pow_py.*/d' requirements.txt

help: ## Prints help for targets with comments
	@cat $(MAKEFILE_LIST) | grep -E '^[a-zA-Z_-]+:.*?## .*$$' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

lint: ## Run linter
	cargo fmt -v --all -- --emit files
	cargo clippy --workspace --tests --all-features
	@./venv/bin/black ./examples/*
	@./venv/bin/black setup.py

test: ## Run tests
	@. ./venv/bin/activate
	$(call	test_pow_sha_256_py)
