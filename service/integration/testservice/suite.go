package testservice

import (
	"net/http"

	"github.com/stretchr/testify/suite"
)

type Suite struct {
	suite.Suite
	service *TestService
}

func (suite *Suite) SetupTest() {
	testservice := New(suite.T())
	suite.service = &testservice
}

func (suite *Suite) TearDownSuite() {
	suite.service.Close(suite.T())
}

// ServeHTTP will inject an HTTP Request into the service and return the response.
func (suite *Suite) ServeHTTP(r *http.Request) *http.Response {
	return suite.service.ServeHTTP(r)
}
