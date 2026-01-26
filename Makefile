.PHONY: help build release test clean

PROJECT_NAME := cascolor
ORGANIZATION := casapps
VERSION := $(shell cat release.txt 2>/dev/null || echo "0.1.0")
DOCKER_IMAGE := rust:latest
BINARIES_DIR := ./binaries
RELEASES_DIR := ./releases

# Platform targets
TARGETS := \
	x86_64-unknown-linux-gnu \
	aarch64-unknown-linux-gnu \
	x86_64-pc-windows-gnu \
	x86_64-apple-darwin \
	aarch64-apple-darwin \
	x86_64-unknown-freebsd

help:
	@echo "cascolor Makefile"
	@echo ""
	@echo "Usage:"
	@echo "  make build     - Build all platforms (outputs to ./binaries)"
	@echo "  make release   - Create GitHub release (outputs to ./releases)"
	@echo "  make test      - Run all tests"
	@echo "  make clean     - Clean build artifacts"
	@echo ""
	@echo "Environment Variables:"
	@echo "  VERSION        - Override version from release.txt"
	@echo ""
	@echo "Current Version: $(VERSION)"

build:
	@echo "Building $(PROJECT_NAME) v$(VERSION) for all platforms..."
	@mkdir -p $(BINARIES_DIR) $(RELEASES_DIR)
	@chmod -R u+w $(BINARIES_DIR) $(RELEASES_DIR) 2>/dev/null || true
	
	@echo "Building x86_64-linux..."
	@docker run --rm --platform linux/amd64 \
		-v "$(PWD)":/workspace \
		-v "$(PWD)/binaries":/output \
		-w /workspace $(DOCKER_IMAGE) bash -c ' \
		apt-get update -qq && \
		apt-get install -y -qq libgtk-4-dev libadwaita-1-dev pkg-config && \
		cargo build --release && \
		strip target/release/$(PROJECT_NAME) 2>/dev/null || true && \
		cp target/release/$(PROJECT_NAME) /output/$(PROJECT_NAME)-linux-x86_64 && \
		chmod 755 /output/$(PROJECT_NAME)-linux-x86_64'
	@echo "✓ Built linux-x86_64"
	
	@echo "Skipping aarch64-linux (requires native ARM64 hardware)"
	@echo "✓ Skipped linux-aarch64"
	
	@echo "Skipping x86_64-windows (GTK4 not supported via MinGW cross-compile)"
	@echo "✓ Skipped windows-x86_64"
	
	@echo "Building x86_64-macos..."
	@echo "⚠ macOS builds require macOS host or osxcross toolchain (skipped)"
	
	@echo "Building aarch64-macos..."
	@echo "⚠ macOS ARM64 builds require macOS host (skipped)"
	
	@echo "Building x86_64-freebsd..."
	@echo "⚠ FreeBSD builds require FreeBSD host or specialized toolchain (skipped)"
	
	@cp $(BINARIES_DIR)/$(PROJECT_NAME)-linux-x86_64 $(BINARIES_DIR)/$(PROJECT_NAME) 2>/dev/null || true
	@echo ""
	@echo "✅ Build complete:"
	@ls -lh $(BINARIES_DIR)/ 2>/dev/null || true

test:
	@echo "Running tests for $(PROJECT_NAME)..."
	@docker run --rm -v "$(PWD)":/workspace -w /workspace $(DOCKER_IMAGE) bash -c ' \
		cargo test --lib --no-fail-fast'

release:
	@echo "Creating release for $(PROJECT_NAME) v$(VERSION)..."
	@mkdir -p $(RELEASES_DIR)
	@chmod -R u+w $(BINARIES_DIR) $(RELEASES_DIR) 2>/dev/null || true
	
	@if ! command -v gh >/dev/null 2>&1; then \
		echo "❌ Error: 'gh' CLI not found. Install from https://cli.github.com/"; \
		exit 1; \
	fi
	
	@echo "Checking if release v$(VERSION) exists..."
	@if gh release view v$(VERSION) >/dev/null 2>&1; then \
		echo "Release v$(VERSION) exists, deleting..."; \
		gh release delete v$(VERSION) -y --cleanup-tag || true; \
	fi
	
	@echo "Copying binaries to releases..."
	@cp $(BINARIES_DIR)/* $(RELEASES_DIR)/ 2>/dev/null || true
	
	@echo "Creating source archive..."
	@docker run --rm \
		-v "$(PWD)":/workspace \
		-v "$(PWD)/releases":/releases \
		-w /workspace $(DOCKER_IMAGE) bash -c ' \
		git archive --format=tar.gz --prefix=$(PROJECT_NAME)-$(VERSION)/ HEAD > /releases/$(PROJECT_NAME)-$(VERSION)-source.tar.gz'
	
	@echo "Creating GitHub release v$(VERSION)..."
	@gh release create v$(VERSION) \
		--title "$(PROJECT_NAME) v$(VERSION)" \
		--notes "Release $(VERSION)" \
		$(RELEASES_DIR)/*
	
	@echo "✅ Release v$(VERSION) created successfully!"
	@echo "Updating release.txt..."
	@echo "$(VERSION)" > release.txt

clean:
	@echo "Cleaning build artifacts..."
	@docker run --rm -v "$(PWD)":/workspace -w /workspace $(DOCKER_IMAGE) bash -c 'rm -rf target/' || true
	@mkdir -p $(BINARIES_DIR) $(RELEASES_DIR)
	@chmod -R u+w $(BINARIES_DIR) $(RELEASES_DIR) 2>/dev/null || true
	@rm -rf $(BINARIES_DIR)/* $(RELEASES_DIR)/* || true
	@echo "✅ Clean complete"
