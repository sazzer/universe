package main

import (
	"log"

	"github.com/kelseyhightower/envconfig"
)

type config struct {
	Debug       bool
	Port        uint16 `default:"8000"`
	DatabaseURL string `envconfig:"DATABASE_URL"`
}

func newConfig() config {
	var c config

	if err := envconfig.Process("", &c); err != nil {
		log.Fatal(err)
	}

	return c
}
