package health

// ComponentHealth represents the health of a single component in the system.
type ComponentHealth struct {
	Error error
}

// SystemHealth represents the health of the entire system.
type SystemHealth struct {
	Components map[string]ComponentHealth
}

// Healthy determines if this component is healthy.
func (c ComponentHealth) Healthy() bool {
	return c.Error == nil
}

// Healthy determines if the entire system is healthy.
// The system is healthy only if every single component is healthy.
func (s SystemHealth) Healthy() bool {
	result := true

	for _, c := range s.Components {
		result = result && c.Healthy()
	}

	return result
}
