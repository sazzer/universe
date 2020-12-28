package server

import "github.com/labstack/echo/v4"

// Interface that endpoints able to mount routes can implement.
type Endpoints interface {
	// Mount the routes for these endpoints.
	Mount(echo *echo.Echo)
}
