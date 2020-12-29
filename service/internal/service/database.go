package service

import (
	"github.com/rs/zerolog/log"
	"github.com/sazzer/universe/service/internal/database"
)

// Create the component for connecting to the database.
func newDatabase(databaseURL string) database.Database {
	log.Debug().Str("url", databaseURL).Msg("Creating database connection")

	db := database.New(databaseURL)

	err := database.Migrate(db)
	if err != nil {
		log.Fatal().Err(err).Msg("Failed to migrate database")
	}

	return db
}
