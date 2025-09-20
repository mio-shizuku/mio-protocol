.PHONY : all help run

CARGO := cargo

all: help

help:
	@echo "Available targets:"
	@echo "  all        - Build all targets (default)"
	@echo "  help       - Show this help message"
	@echo "  test	   - Run tests"
	@echo "  clean      - Clean the project"

BUILD_TYPE := release

CARGO_ARGS := --$(BUILD_TYPE)

test:
	@$(CARGO) test $(CARGO_ARGS)

clean:
	@$(CARGO) clean

%:
	@$(CARGO) $@
