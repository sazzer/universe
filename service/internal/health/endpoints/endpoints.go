package endpoints

import (
	"github.com/labstack/echo/v4"
	"github.com/sazzer/universe/service/internal/health"
	"github.com/sazzer/universe/service/internal/response"
)

// The endponts for checking the system health.
type Endpoints struct {
	healthcheckUseCase health.HealthcheckUseCase
}

// Create a new endpoints instance.
func New(healthcheckUseCase health.HealthcheckUseCase) Endpoints {
	return Endpoints{healthcheckUseCase}
}

// Mount the routes for these endpoints.
func (e Endpoints) Mount(echo *echo.Echo) {
	echo.GET("/health", response.WrapHandler(e.CheckHealth))
}
