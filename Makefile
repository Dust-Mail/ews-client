VERSION := 0.2.0

.PHONY: release

release:
	sed -i 's/^version = .*/version = "$(VERSION)"/' Cargo.toml
	cargo fetch
	git commit -am "Bump version to $(VERSION)"
	git tag -a v$(VERSION) -m "Version $(VERSION)"
	git push && git push --tags