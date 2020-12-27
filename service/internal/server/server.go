package server

import (
	"fmt"

	"github.com/rs/zerolog/log"
)

// Server represents the HTTP server that is processing incoming requests.
type Server struct{}

// New will create a new instance of the server ready to run.
func New() Server {
	log.Info().Msg("Building HTTP Server")

	return Server{}
}

// Start the service listening on the provided HTTP port.
func (s Server) Start(port uint16) {
	address := fmt.Sprintf(":%d", port)
	log.Debug().Str("address", address).Msg("Starting server")
}
