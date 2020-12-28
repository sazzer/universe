package service_test

import (
	"context"
	"errors"
	"testing"

	"github.com/sazzer/universe/service/internal/health"
	"github.com/sazzer/universe/service/internal/health/service"
	"github.com/stretchr/testify/assert"
)

var errAnyError = errors.New("Oops")

type MockComponent struct {
	error error
}

func (m MockComponent) CheckHealth(ctx context.Context) error {
	return m.error
}

func TestEmptySystem(t *testing.T) {
	service := service.New(map[string]health.Component{})
	health := service.CheckHealth(context.Background())

	assert.True(t, health.Healthy())
	assert.Empty(t, health.Components)
}

func TestHealthySystem(t *testing.T) {
	service := service.New(map[string]health.Component{
		"healthy": MockComponent{nil},
	})
	health := service.CheckHealth(context.Background())

	assert.True(t, health.Healthy())
	assert.Equal(t, 1, len(health.Components))
	assert.Equal(t, nil, health.Components["healthy"].Error)
}

func TestUnhealthySystem(t *testing.T) {
	service := service.New(map[string]health.Component{
		"unhealthy": MockComponent{errAnyError},
	})
	health := service.CheckHealth(context.Background())

	assert.False(t, health.Healthy())
	assert.Equal(t, 1, len(health.Components))
	assert.Equal(t, errAnyError, health.Components["unhealthy"].Error)
}

func TestMixedSystem(t *testing.T) {
	service := service.New(map[string]health.Component{
		"unhealthy": MockComponent{errAnyError},
		"healthy":   MockComponent{nil},
	})
	health := service.CheckHealth(context.Background())

	assert.False(t, health.Healthy())
	assert.Equal(t, 2, len(health.Components))
	assert.Equal(t, nil, health.Components["healthy"].Error)
	assert.Equal(t, errAnyError, health.Components["unhealthy"].Error)
}
