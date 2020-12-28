package service

import (
	"net/http"
)

// ServeHTTP will inject an HTTP Request into the service and return the response.
func (s Service) ServeHTTP(r *http.Request) *http.Response {
	return s.server.ServeHTTP(r)
}
