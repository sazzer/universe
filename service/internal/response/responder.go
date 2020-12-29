package response

import (
	"github.com/labstack/echo/v4"
)

// Interface that anything that represents an HTTP Response can implement.
type Responder interface {
	// Render the HTTP Response onto the provided Echo context.
	Render(c echo.Context) error
}

// Function type that represents a handler function.
type HandlerFunc func(c echo.Context) Responder

// Wrap one of our handler functions in one that implements the Echo interface.
func WrapHandler(handler HandlerFunc) echo.HandlerFunc {
	return func(c echo.Context) error {
		response := handler(c)

		return response.Render(c)
	}
}
