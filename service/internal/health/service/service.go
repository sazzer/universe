package service

import (
	"context"

	"github.com/sazzer/universe/service/internal/health"
)

// The Health Service, able to check the health of the entire system.
type HealthService struct {
	components map[string]health.Component
}

// Create a new instance of the Health Service.
func New(components map[string]health.Component) HealthService {
	return HealthService{components}
}

// CheckHealth will check the health of the system and return the details of all the components checked.
func (h HealthService) CheckHealth(ctx context.Context) health.SystemHealth {
	components := map[string]health.ComponentHealth{}

	for name, component := range h.components {
		result := component.CheckHealth(ctx)
		components[name] = health.ComponentHealth{Error: result}
	}

	return health.SystemHealth{Components: components}
}
