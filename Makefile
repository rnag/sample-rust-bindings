.PHONY: all

all:
	@echo "Run my targets individually!"

venv: venv/touchfile

venv/touchfile: requirements-dev.txt
	test -d venv || virtualenv venv
	. venv/bin/activate; pip install -Ur requirements-dev.txt
	touch venv/touchfile

.PHONY: develop
develop: venv
	. venv/bin/activate; maturin develop

.PHONY: test
test: develop
	. venv/bin/activate; pytest tests

.PHONY: build
build: venv
	. venv/bin/activate; maturin build

.PHONY: dist
dist: venv
	( \
		. venv/bin/activate; \
		docker run --rm -v $(shell pwd):/io konstin2/maturin build --release --strip; \
	)
