package testservice

import (
	"fmt"
	"net/http"

	"github.com/stretchr/testify/suite"
)

type Suite struct {
	suite.Suite
	service *TestService
}

func (suite *Suite) SetupTest() {
	fmt.Println("Setting up suite")

	testservice := New(suite.T())
	suite.service = &testservice
}

func (suite *Suite) TearDownSuite() {
	fmt.Println("Tearing down suite")

	suite.service.Close(suite.T())
}

// ServeHTTP will inject an HTTP Request into the service and return the response.
func (suite *Suite) ServeHTTP(r *http.Request) *http.Response {
	return suite.service.ServeHTTP(r)
}
