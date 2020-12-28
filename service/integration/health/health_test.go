package health_test

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/sazzer/universe/service/integration/testservice"
	"github.com/stretchr/testify/suite"
)

type Suite struct {
	testservice.Suite
}

func (suite *Suite) TestHealth() {
	res := suite.ServeHTTP(httptest.NewRequest("GET", "/health", nil))
	defer res.Body.Close()

	suite.Assertions.Equal(http.StatusNotFound, res.StatusCode)
}

func TestHealthSuite(t *testing.T) {
	suite.Run(t, new(Suite))
}
