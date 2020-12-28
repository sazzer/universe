package service

import "github.com/sazzer/universe/service/internal/server"

// Create the component for the HTTP Server.
func newServer() server.Server {
	return server.New()
}
