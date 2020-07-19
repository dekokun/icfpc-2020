PHONY: relase
release:
	cargo vendor
	git add .
	git commit -m'#release' --allow-empty
	git push origin submission
PHONY: build
build:
	./build.sh