package database

import (
	"context"
	"errors"
	"fmt"

	"github.com/jackc/pgx/v4"
	"github.com/rs/zerolog/log"
)

type Transaction struct {
	pgx.Tx
}

func (d Database) NewTransaction(ctx context.Context) (Transaction, error) {
	tx, err := d.pool.Begin(ctx)
	if err != nil {
		log.Error().Err(err).Msg("An error occurred creating a database transaction")

		return Transaction{}, fmt.Errorf("failed to create transaction: %w", err)
	}

	return Transaction{tx}, nil
}

// Rollback the transaction, logging out any unexpected errors.
// If the error is that the transaction is already closed then this is acceptable.
func (t Transaction) Rollback(ctx context.Context) {
	err := t.Tx.Rollback(ctx)
	if err != nil && !errors.Is(err, pgx.ErrTxClosed) {
		log.Error().Err(err).Msg("Failed to roll back transaction")
	}
}
