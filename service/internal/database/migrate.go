//go:generate packr2
package database

import (
	"context"
	"fmt"

	"github.com/gobuffalo/packd"
	"github.com/gobuffalo/packr/v2"
	"github.com/rs/zerolog/log"
)

// Migrate the database to the latest version of the schema.
func Migrate(db Database) error {
	log.Debug().Msg("Migrating database")

	ctx := context.Background()

	tx, err := db.NewTransaction(ctx)
	if err != nil {
		return fmt.Errorf("failed to create transaction: %w", err)
	}

	defer tx.Rollback(ctx)

	err = lockMigrationsTable(ctx, tx)
	if err != nil {
		return err
	}

	applied, err := listAppliedMigrations(ctx, tx)
	if err != nil {
		return err
	}

	log.Debug().Strs("migrations", applied).Msg("Applied migrations")

	box := packr.New("Migrations", "../../migrations")

	migrations, err := listMigrations(box)
	if err != nil {
		return err
	}

	log.Debug().Strs("migrations", migrations).Msg("Available migrations")

	count := 0

	for _, migration := range migrations {
		if !contains(applied, migration) {
			count++

			err = applyMigration(ctx, tx, box, migration)
			if err != nil {
				return err
			}
		}
	}

	log.Debug().Int("applied", count).Int("outstanding", len(migrations)-len(applied)).Int("total", len(migrations)).
		Msg("Applied migrations")

	err = tx.Commit(ctx)
	if err != nil {
		log.Error().Err(err).Msg("Failed to commit database transaction")

		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}

// Ensure that the migrations table exists and take out an exclusive lock on it.
// The exclusive lock helps to ensure that only one process is peforming migrations at any given time.
func lockMigrationsTable(ctx context.Context, tx Transaction) error {
	log.Trace().Msg("Creating migrations table")

	_, err := tx.Exec(ctx, `CREATE TABLE IF NOT EXISTS __migrations(
		migration_file TEXT PRIMARY KEY,
		sequence SERIAL NOT NULL,
		executed TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
		executed_from TEXT NOT NULL DEFAULT inet_client_addr()
	)`)
	if err != nil {
		log.Error().Err(err).Msg("Failed to create migrations table")

		return fmt.Errorf("error creating migrations table: %w", err)
	}

	_, err = tx.Exec(ctx, `LOCK TABLE __migrations IN EXCLUSIVE MODE`)
	if err != nil {
		log.Error().Err(err).Msg("Failed to lock migrations table")

		return fmt.Errorf("error locking migrations table: %w", err)
	}

	return nil
}

// List all of the migrations that have previously been applied.
func listAppliedMigrations(ctx context.Context, tx Transaction) ([]string, error) {
	log.Trace().Msg("Listing previously applied migrations")

	rows, err := tx.Query(ctx, "SELECT migration_file FROM __migrations")
	if err != nil {
		log.Error().Err(err).Msg("Failed to list applied migrations")

		return nil, fmt.Errorf("error listing applied migrations: %w", err)
	}
	defer rows.Close()

	var result []string

	for rows.Next() {
		var migration string

		err = rows.Scan(&migration)
		if err != nil {
			log.Error().Err(err).Msg("Failed to read migration row")

			return nil, fmt.Errorf("error reading migration row: %w", err)
		}

		log.Trace().Str("migration", migration).Msg("Listed migration")

		result = append(result, migration)
	}

	return result, nil
}

// List all of the migrations that could potentially be applied.
func listMigrations(box packd.Walker) ([]string, error) {
	log.Trace().Msg("Listing migration files")

	var result []string

	err := box.Walk(func(path string, file packr.File) error {
		log.Trace().Str("path", path).Msg("Found migration file")

		result = append(result, path)

		return nil
	})
	if err != nil {
		log.Error().Err(err).Msg("Failed to list migration files")

		return nil, fmt.Errorf("error listing migration files: %w", err)
	}

	return result, nil
}

// Apply the migration from the given file.
func applyMigration(ctx context.Context, tx Transaction, box *packr.Box, file string) error {
	contents, err := box.FindString(file)
	if err != nil {
		log.Error().Err(err).Str("file", file).Msg("Failed to load migration")

		return fmt.Errorf("error reading migration file: %w", err)
	}

	_, err = tx.Exec(ctx, contents)
	if err != nil {
		log.Error().Err(err).Str("file", file).Msg("Failed to execute migration")

		return fmt.Errorf("error applying migration: %w", err)
	}

	_, err = tx.Exec(ctx, "INSERT INTO __migrations(migration_file) VALUES ($1)", file)
	if err != nil {
		log.Error().Err(err).Str("file", file).Msg("Failed to record migration")

		return fmt.Errorf("error recording migration: %w", err)
	}

	return nil
}

// Helper to find if the given string is present in the given slice.
func contains(input []string, find string) bool {
	for _, v := range input {
		if v == find {
			return true
		}
	}

	return false
}
