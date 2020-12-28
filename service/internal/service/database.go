package service

import (
	"github.com/rs/zerolog/log"
	"github.com/sazzer/universe/service/internal/database"
)

// Create the component for connecting to the database.
func newDatabase(databaseURL string) database.Database {
	log.Debug().Str("url", databaseURL).Msg("Creating database connection")

	database := database.New(databaseURL)

	return database
}
