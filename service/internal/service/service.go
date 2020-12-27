package service

import (
	"github.com/rs/zerolog/log"
	"github.com/sazzer/universe/service/internal/server"
)

// Service represents the actual service that is performing all of the work.
type Service struct {
	server server.Server
}

// New will create a new instance of the service ready to run.
func New() Service {
	log.Info().Msg("Building Universe")

	server := server.New()

	log.Info().Msg("Built Universe")

	return Service{
		server,
	}
}

// Start the service listening on the provided HTTP port.
func (s Service) Start(port uint16) {
	s.server.Start(port)
}
