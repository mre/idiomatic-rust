.PHONY: check
check:
	cargo check

.PHONY: build
build: check
	docker build -t mre0/idiomatic-rust-ci .

.PHONY: push
push: build
	docker push mre0/idiomatic-rust-ci