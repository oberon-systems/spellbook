# OS Params
ARCH = $(shell uname -m)
OS = $(shell uname -s | tr '[:upper:]' '[:lower:]')


# Temporary files location for .bashrc
TEMP := $(shell mktemp /tmp/oberon-systems-bashrc.XXXXXX)

# Python VENV Params
VIRTUAL_ENV = venv

# UV Params
UV = $(PWD)/.apps/uv/uv
UV_URL = https://github.com/astral-sh/uv/releases/latest/download/uv-$(ARCH)-unknown-$(OS)-gnu.tar.gz

# Pre-commit configs
PRE_COMMIT_HOME = $(PWD)/.pre-commit

# Export env params
export VIRTUAL_ENV
export PRE_COMMIT_HOME

.install-uv:
	@mkdir -p .apps/uv
	@curl -LsSf $(UV_URL) | tar -xz -C .apps/uv --strip-components=1

.install-venv:
	@if [ ! -f $(UV) ]; then \
  		echo "WARNING: uv binary nit found, installing..."; \
  		$(MAKE) .install-uv; \
  	fi
	@$(UV) venv --relocatable --python python3.12
	@$(UV) pip install -r requirements.txt

enable:
	@if [ ! -f ".venv/bin/python" ]; then \
		"WARNING: virtual env not found, installing..."; \
  		$(MAKE) .install-venv; \
  	fi
	@echo "Starting Oberon Systems Developing Environment..."
	@cat $(HOME)/.bashrc > $(TEMP)
	# export uv path
	@echo 'export PATH="$(PWD)/.apps/uv:$$PATH"' >> $(TEMP)
	# enable venv and pre-commit
	@echo "source $(PWD)/.venv/bin/activate" >> $(TEMP)
	@echo 'export PRE_COMMIT_HOME="$(PWD)/.pre-commit"' >> $(TEMP)
	# install pre-commit
	@echo '' >> $(TEMP)
	@echo '# Auto-install pre-commit hooks' >> $(TEMP)
	@echo 'if [ ! -d .pre-commit ]; then' >> $(TEMP)
	@echo '  echo "Pre-commit not found, installing..."' >> $(TEMP)
	@echo '  pre-commit install-hooks' >> $(TEMP)
	@echo '  pre-commit install' >> $(TEMP)
	@echo 'fi' >> $(TEMP)
	@cat motd
	@/usr/bin/env bash --init-file $(TEMP)
	@rm -fv $(TEMP)
