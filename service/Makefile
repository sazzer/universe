.DEFAULT_GOAL := integration

prepare:

tidy:
	go get -u ./...
	go mod tidy

run: prepare
	go run ./cmd/universe/*

build: prepare
	go build -o output/universe ./cmd/universe/*

lint:
	golangci-lint run
	
test: prepare lint
	go test ${GOTEST_FLAGS} ./internal/...

integration: prepare lint test
	go test ${GOTEST_FLAGS} ./integration/...