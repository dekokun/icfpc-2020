PHONY: relase
release:
	cargo vendor
	git add .
	git commit -m'#release'
	git push origin submission
PHONY: build
build:
	./build.sh