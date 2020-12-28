package server

import (
	"net/http"
	"net/http/httptest"
)

// ServeHTTP will inject an HTTP Request into the service and return the response.
func (s Server) ServeHTTP(r *http.Request) *http.Response {
	w := httptest.NewRecorder()

	s.e.ServeHTTP(w, r)

	return w.Result()
}
