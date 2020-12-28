package testservice

import (
	"context"
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/testcontainers/testcontainers-go"
	"github.com/testcontainers/testcontainers-go/wait"
)

// Wrapper around the Postgres docker container.
type postgres struct {
	container testcontainers.Container
}

// Create a new instance of the Postgres docker container.
func newPostgresContainer(t *testing.T) postgres {
	ctx := context.Background()

	req := testcontainers.ContainerRequest{
		Image:        "postgres:12.4-alpine",
		ExposedPorts: []string{"5432/tcp"},
		Env: map[string]string{
			"POSTGRES_DB":       "universe_test",
			"POSTGRES_USER":     "universe_test",
			"POSTGRES_PASSWORD": "universe_test",
		},
		WaitingFor: wait.ForListeningPort("5432/tcp"),
	}

	container, err := testcontainers.GenericContainer(ctx, testcontainers.GenericContainerRequest{
		ContainerRequest: req,
		Started:          true,
	})
	assert.NoError(t, err)

	return postgres{container}
}

// Close the Postgres docker container.
func (p postgres) close(t *testing.T) {
	ctx := context.Background()

	err := p.container.Terminate(ctx)
	assert.NoError(t, err)
}

// Get the URL to the postgres docker container.
func (p postgres) url(t *testing.T) string {
	ctx := context.Background()

	ip, err := p.container.Host(ctx)
	assert.NoError(t, err)

	port, err := p.container.MappedPort(ctx, "5432/tcp")
	assert.NoError(t, err)

	return fmt.Sprintf("postgres://universe_test:universe_test@%s:%s/universe_test", ip, port.Port())
}
