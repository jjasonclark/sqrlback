EXE_TARGET = sqrlback
DEFAULT_TARGET = target/debug/$(EXE_TARGET)
SOURCE_FILES = $(git ls-files -- src/)

# all: $(DEFAULT_TARGET)

.PHONY: build clean test

target/debug/$(EXE_TARGET): $(SOURCE_FILES)
	cargo build

clean:
	cargo clean

test:
	cargo test
