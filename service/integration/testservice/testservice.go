package testservice

import (
	"net/http"
	"testing"

	"github.com/sazzer/universe/service/internal/service"
	"github.com/sazzer/universe/service/testutils"
)

// Wrapper around the Universe service for testing against.
type TestService struct {
	service  service.Service
	postgres testutils.Postgres
}

// New will create a new Test Service.
func New(t *testing.T) TestService {
	t.Helper()

	postgres := testutils.NewPostgresContainer(t)

	postgresURL := postgres.URL(t)

	service := service.New(postgresURL)

	return TestService{service, postgres}
}

// Close will close the test service down.
func (s TestService) Close(t *testing.T) {
	t.Helper()

	s.postgres.Close(t)
}

// ServeHTTP will inject an HTTP Request into the service and return the response.
func (s TestService) ServeHTTP(r *http.Request) *http.Response {
	return s.service.ServeHTTP(r)
}
