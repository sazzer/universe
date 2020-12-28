package testservice

import (
	"net/http"
	"testing"

	"github.com/sazzer/universe/service/internal/service"
)

// Wrapper around the Universe service for testing against.
type TestService struct {
	service  service.Service
	postgres postgres
}

// New will create a new Test Service.
func New(t *testing.T) TestService {
	postgres := newPostgresContainer(t)

	postgresURL := postgres.url(t)

	service := service.New(postgresURL)

	return TestService{service, postgres}
}

// Close will close the test service down.
func (s TestService) Close(t *testing.T) {
	s.postgres.close(t)
}

// ServeHTTP will inject an HTTP Request into the service and return the response.
func (s TestService) ServeHTTP(r *http.Request) *http.Response {
	return s.service.ServeHTTP(r)
}
