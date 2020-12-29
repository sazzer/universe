package database

import (
	"context"
	"fmt"
)

func (d Database) CheckHealth(ctx context.Context) error {
	_, err := d.pool.Exec(ctx, "SELECT 1")

	if err != nil {
		return fmt.Errorf("%w", err)
	}

	return nil
}
