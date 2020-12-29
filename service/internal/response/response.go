package response

import (
	"net/http"

	"github.com/labstack/echo/v4"
)

// Simple HTTP Response.
type Response struct {
	statusCode int
	payload    interface{}
}

// Create a new Response wrapper for the provided payload.
func NewResponse(payload interface{}) Response {
	return Response{
		statusCode: http.StatusOK,
		payload:    payload,
	}
}

// Specify the status code to use for the response.
func (r Response) Status(statusCode int) Response {
	r.statusCode = statusCode

	return r
}

func (r Response) Render(c echo.Context) error {
	return c.JSON(r.statusCode, r.payload)
}
