BIN_NAME := wstick-packer
PARAMS := packs/*
HT_VIEW_FILE = $(shell ls -t $(PROFILE_DIR)/heaptrack.$(BIN_NAME).*.gz | head -n1)
HT_COMPARE_FILES = $(shell ls -t $(PROFILE_DIR)/heaptrack.$(BIN_NAME).*.gz | head -n2)
MASSIF_VIEW_FILE = $(shell ls -t $(PROFILE_DIR)/massif.out.* | head -n1)

# includes debugging symbols on profiling releases
ifeq ($(filter gh-release,$(MAKECMDGOALS)),)
RUSTFLAGS=-g
TARGET ?= debug
else
TARGET = release
endif

PROFILE_DIR := target/$(TARGET)/profile
EXECUTABLE := target/$(TARGET)/$(BIN_NAME)

.PHONY: gh-release
gh-release: $(BIN_NAME).tar.gz

$(BIN_NAME).tar.gz: target/release/$(BIN_NAME)
	$(info gzipping $(BIN_NAME) to $(BIN_NAME).tar.gz...)
	@cd target/release/ && \
		tar -czf $(BIN_NAME).tar.gz $(BIN_NAME)
	$(info release file is ready on target/release/$(BIN_NAME).tar.gz)

.PHONY: profile-clean
profile-clean: ht-clean massif-clean

.PHONY: ht
ht: ht-build ht-view

.PHONY: ht-build
ht-build: $(EXECUTABLE)
	@mkdir -p $(PROFILE_DIR)
	@heaptrack $(EXECUTABLE) $(PARAMS)
	@mv -t $(PROFILE_DIR) heaptrack.$(BIN_NAME).*.gz

.PHONY: ht-view
ht-view:
	@heaptrack --analyze $(HT_VIEW_FILE)

.PHONY: ht-compare
ht-compare:
	@heaptrack --analyze -d $(HT_COMPARE_FILES)

.PHONY: ht-clean
ht-clean:
	-@rm -f $(PROFILE_DIR)/heaptrack.$(BIN_NAME).*.gz

.PHONY: massif
massif: massif-build massif-view

.PHONY: massif-build
massif-build: $(EXECUTABLE)
	@mkdir -p $(PROFILE_DIR)
	@valgrind --tool=massif $(EXECUTABLE) $(PARAMS)
	@mv -f -t $(PROFILE_DIR) massif.out.*

.PHONY: massif-view
massif-view:
	@massif-visualizer $(MASSIF_VIEW_FILE)

.PHONY: massif-clean
massif-clean:
	-@rm -f $(PROFILE_DIR)/massif.out*

debug: target/debug/$(BIN_NAME)

release: target/release/$(BIN_NAME)

target/debug/$(BIN_NAME):
	@cargo build

target/release/$(BIN_NAME):
	@cargo build --release
