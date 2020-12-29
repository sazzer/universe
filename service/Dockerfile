FROM golang:1.15.6-alpine3.12 AS builder

ENV CGO_ENABLED=0

WORKDIR /universe

COPY . .

RUN go get -u github.com/gobuffalo/packr/v2/packr2
RUN go get ./...
RUN go generate ./...
RUN go build -o output/universe ./cmd/universe/*



FROM alpine:3.12.3

RUN apk update && apk add wget

ENV DOCKERIZE_VERSION v0.6.1
RUN wget https://github.com/jwilder/dockerize/releases/download/$DOCKERIZE_VERSION/dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
  && tar -C /usr/local/bin -xzvf dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
  && rm dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz

WORKDIR /universe

ENV PORT=8000
ENV DATABASE_URL=
ENV DEBUG=false

COPY --from=builder /universe/output/universe /universe/universe
COPY docker/start.sh /universe

CMD ["/universe/start.sh"]

HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 CMD wget -q http://localhost:$PORT/health