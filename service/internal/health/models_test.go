package health_test

import (
	"errors"
	"testing"

	"github.com/sazzer/universe/service/internal/health"
	"github.com/stretchr/testify/assert"
)

var errAnyError = errors.New("Oops")

func TestHealthyComponent(t *testing.T) {
	component := health.ComponentHealth{Error: nil}

	assert.True(t, component.Healthy())
}

func TestUnhealthyComponent(t *testing.T) {
	component := health.ComponentHealth{Error: errAnyError}

	assert.False(t, component.Healthy())
}

func TestEmptySystem(t *testing.T) {
	system := health.SystemHealth{Components: map[string]health.ComponentHealth{}}

	assert.True(t, system.Healthy())
}

func TestHealthySystem(t *testing.T) {
	system := health.SystemHealth{Components: map[string]health.ComponentHealth{
		"healthy": {Error: nil},
	}}

	assert.True(t, system.Healthy())
}

func TestUnhealthySystem(t *testing.T) {
	system := health.SystemHealth{Components: map[string]health.ComponentHealth{
		"unhealthy": {Error: errAnyError},
	}}

	assert.False(t, system.Healthy())
}

func TestMixedSystem(t *testing.T) {
	system := health.SystemHealth{Components: map[string]health.ComponentHealth{
		"healthy":   {Error: nil},
		"unhealthy": {Error: errAnyError},
	}}

	assert.False(t, system.Healthy())
}
