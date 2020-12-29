package endpoints

import (
	"net/http"

	"github.com/sazzer/universe/service/internal/health"
	"github.com/sazzer/universe/service/internal/response"
)

// HTTP Model representation of the health of a single component.
type ComponentHealthModel struct {
	Healthy bool   `json:"healthy"`
	Message string `json:"message,omitempty"`
}

// HTTP Model representation of the health of the entire system.
type SystemHealthModel struct {
	Healthy    bool                            `json:"healthy"`
	Components map[string]ComponentHealthModel `json:"components"`
}

// Build the HTTP response for the provided system health.
func buildResponse(health health.SystemHealth) response.Responder {
	components := map[string]ComponentHealthModel{}

	for name, component := range health.Components {
		componentHealth := ComponentHealthModel{
			Healthy: component.Healthy(),
		}

		if component.Error != nil {
			componentHealth.Message = component.Error.Error()
		}

		components[name] = componentHealth
	}

	system := SystemHealthModel{
		Healthy:    health.Healthy(),
		Components: components,
	}

	result := response.NewResponse(system)

	if system.Healthy {
		result = result.Status(http.StatusOK)
	} else {
		result = result.Status(http.StatusServiceUnavailable)
	}

	return result
}
