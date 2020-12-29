package endpoints

import (
	"time"

	"github.com/labstack/echo/v4"
	"github.com/sazzer/universe/service/internal/response"
	"golang.org/x/net/context"
)

// Endpoint for checking the health of the system.
func (e Endpoints) CheckHealth(c echo.Context) response.Responder {
	ctx, cancel := context.WithTimeout(c.Request().Context(), 5*time.Second)
	defer cancel()

	health := e.healthcheckUseCase.CheckHealth(ctx)

	return buildResponse(health)
}
