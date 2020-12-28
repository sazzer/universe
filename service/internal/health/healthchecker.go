package health

import "context"

// Interface that any component able to check it's own health can implement.
type Component interface {
	// CheckHealth will check the health of the component, returning an error if the component is unhealthy.
	CheckHealth(ctx context.Context) error
}
