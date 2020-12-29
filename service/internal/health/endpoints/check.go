package endpoints

import (
	"time"

	"github.com/labstack/echo/v4"
	"github.com/sazzer/universe/service/internal/response"
	"golang.org/x/net/context"
)

// Timeout for the healthchecks.
const timeout = 5 * time.Second

// Endpoint for checking the health of the system.
func (e Endpoints) CheckHealth(c echo.Context) response.Responder {
	ctx, cancel := context.WithTimeout(c.Request().Context(), timeout)
	defer cancel()

	health := e.healthcheckUseCase.CheckHealth(ctx)

	return buildResponse(health)
}
