package main

import (
	"log"

	"github.com/kelseyhightower/envconfig"
)

type config struct {
	Debug bool
}

func newConfig() config {
	var c config

	if err := envconfig.Process("", &c); err != nil {
		log.Fatal(err)
	}

	return c
}
