package testservice

import (
	"net/http"

	"github.com/sazzer/universe/service/internal/service"
)

// Wrapper around the Universe service for testing against.
type TestService struct {
	service service.Service
}

// New will create a new Test Service.
func New() TestService {
	service := service.New()

	return TestService{service}
}

// Close will close the test service down.
func (s TestService) Close() {
}

// ServeHTTP will inject an HTTP Request into the service and return the response.
func (s TestService) ServeHTTP(r *http.Request) *http.Response {
	return s.service.ServeHTTP(r)
}
