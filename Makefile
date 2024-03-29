COMMENT := ""
PHONY: relase
release:
	# cargo vendor
	git add .
	git commit -m'$(COMMENT) #release' --allow-empty
	git push origin submission
PHONY: build
build:
	./build.sh
PHONY: test
test:
	cargo test -- --nocapture
