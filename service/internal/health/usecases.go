package health

import "context"

// Use Case used to check the health of the system.
type HealthcheckUseCase interface {
	// CheckHealth will check the health of the system and return the details of all the components checked.
	CheckHealth(ctx context.Context) SystemHealth
}
