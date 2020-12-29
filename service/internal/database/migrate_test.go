package database_test

import (
	"testing"

	"github.com/sazzer/universe/service/internal/database"
	"github.com/sazzer/universe/service/testutils"
	"github.com/stretchr/testify/assert"
)

func TestMigrate(t *testing.T) {
	container := testutils.NewPostgresContainer(t)
	defer container.Close(t)

	db := database.New(container.URL(t))

	err := database.Migrate(db)
	assert.NoError(t, err)
}

func TestMigrateTwice(t *testing.T) {
	container := testutils.NewPostgresContainer(t)
	defer container.Close(t)

	db := database.New(container.URL(t))

	err := database.Migrate(db)
	assert.NoError(t, err)

	err = database.Migrate(db)
	assert.NoError(t, err)
}
