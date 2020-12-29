package endpoints

import (
	"github.com/labstack/echo/v4"
	"github.com/sazzer/universe/service/internal/response"
)

// Endpoint for checking the health of the system.
func (e Endpoints) CheckHealth(c echo.Context) response.Responder {
	health := e.healthcheckUseCase.CheckHealth(c.Request().Context())

	return buildResponse(health)
}
