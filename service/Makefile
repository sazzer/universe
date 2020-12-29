.DEFAULT_GOAL := integration
.PHONY: docker tidy setup

setup:
	go get -u github.com/gobuffalo/packr/v2/packr2
	
prepare:
	go generate ./...

tidy:
	go mod tidy

run: prepare
	go run ./cmd/universe/*

build: prepare
	go build -o output/universe ./cmd/universe/*

docker:
	docker build -t universe-service:latest .

lint:
	golangci-lint run
	
clearcache:
	go clean -testcache

test: prepare lint
	go test -v ${GOTEST_FLAGS} ./internal/...

integration: prepare lint test
	go test -v ${GOTEST_FLAGS} ./integration/...
