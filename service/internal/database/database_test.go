package database_test

import (
	"context"
	"testing"

	"github.com/sazzer/universe/service/internal/database"
	"github.com/sazzer/universe/service/testutils"
	"github.com/stretchr/testify/assert"
)

func TestDatabaseWrapper(t *testing.T) {
	container := testutils.NewPostgresContainer(t)
	defer container.Close(t)

	db := database.New(container.URL(t))

	ctx := context.Background()

	tx, err := db.NewTransaction(ctx)
	assert.NoError(t, err)

	defer tx.Rollback(ctx)

	_, err = tx.Exec(ctx, "SELECT 1")
	assert.NoError(t, err)
}
