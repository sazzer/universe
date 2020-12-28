package endpoints

import (
	"net/http"

	"github.com/labstack/echo/v4"
	"github.com/rs/zerolog/log"
)

// Endpoint for checking the health of the system.
func (e Endpoints) CheckHealth(c echo.Context) error {
	health := e.healthcheckUseCase.CheckHealth(c.Request().Context())

	log.Info().Interface("health", health).Msg("System health")

	return c.String(http.StatusOK, "Hello, World!")
}
