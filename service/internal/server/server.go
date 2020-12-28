package server

import (
	"fmt"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"github.com/rs/zerolog/log"
)

// Server represents the HTTP server that is processing incoming requests.
type Server struct {
	e *echo.Echo
}

// New will create a new instance of the server ready to run.
func New() Server {
	log.Info().Msg("Building HTTP Server")

	e := echo.New()

	e.Use(middleware.Logger())
	e.Use(middleware.Recover())
	e.Use(middleware.CORSWithConfig(middleware.CORSConfig{
		ExposeHeaders: []string{"link"},
	}))
	e.Use(middleware.Gzip())
	e.Use(middleware.RequestID())

	return Server{e}
}

// Start the service listening on the provided HTTP port.
func (s Server) Start(port uint16) {
	address := fmt.Sprintf(":%d", port)
	log.Debug().Str("address", address).Msg("Starting HTTP server")

	err := s.e.Start(address)
	if err != nil {
		log.Fatal().Str("address", address).Err(err).Msg("Failed to start HTTP server")
	}
}
