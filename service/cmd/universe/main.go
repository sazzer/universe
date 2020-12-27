package main

import (
	"os"
	"time"

	_ "github.com/joho/godotenv/autoload"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
)

func main() {
	config := newConfig()

	zerolog.TimeFieldFormat = zerolog.TimeFormatUnix

	if config.Debug {
		zerolog.SetGlobalLevel(zerolog.DebugLevel)

		log.Logger = log.Output(zerolog.ConsoleWriter{Out: os.Stderr, TimeFormat: time.RFC3339}).With().Caller().Logger()
	} else {
		log.Logger = zerolog.New(os.Stderr).With().Timestamp().Caller().Logger()
	}

	log.Info().Msg("Hello")
}
