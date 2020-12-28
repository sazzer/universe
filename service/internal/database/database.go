package database

import (
	"context"

	"github.com/jackc/pgx/v4/pgxpool"
	"github.com/rs/zerolog/log"
)

// Database represents a connaction to the Postgres database.
type Database struct {
	pool *pgxpool.Pool
}

// New creates a new Database connection.
func New(url string) Database {
	log.Debug().Str("url", url).Msg("Connecting to database")

	pool, err := pgxpool.Connect(context.Background(), url)
	if err != nil {
		log.Fatal().Err(err).Str("url", url).Msg("Failed to open connection pool")
	}

	return Database{pool}
}
