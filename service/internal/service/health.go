package service

import (
	"github.com/rs/zerolog/log"
	"github.com/sazzer/universe/service/internal/health"
	"github.com/sazzer/universe/service/internal/health/endpoints"
	"github.com/sazzer/universe/service/internal/health/service"
)

// The Health component for the service.
type healthComponent struct {
	Endpoints endpoints.Endpoints
	Service   service.HealthService
}

// Create a new Health Component.
func newHealthComponent(components map[string]health.Component) healthComponent {
	log.Debug().Msg("Creating health service")

	service := service.New(components)
	endpoints := endpoints.New(service)

	return healthComponent{
		Service:   service,
		Endpoints: endpoints,
	}
}
