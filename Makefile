EXE_TARGET = sqrlback
BUILD_PATH = target
STAGE = debug
DEFAULT_TARGET = $(BUILD_PATH)/$(STAGE)/$(EXE_TARGET)
SOURCE_FILES = $(git ls-files -- src/)

# all: $(DEFAULT_TARGET)

.PHONY: build clean test

$(DEFAULT_TARGET): $(SOURCE_FILES)
	cargo build

build:
	cargo clean

clean:
	cargo clean

test:
	cargo test
