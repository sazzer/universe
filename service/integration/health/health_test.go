package health_test

import (
	"bytes"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/bradleyjkemp/cupaloy"
	"github.com/sazzer/universe/service/integration/testservice"
	"github.com/stretchr/testify/suite"
)

type Suite struct {
	testservice.Suite
}

func (suite *Suite) TestHealth() {
	res := suite.ServeHTTP(httptest.NewRequest("GET", "/health", nil))
	defer res.Body.Close()

	suite.Assertions.Equal(http.StatusOK, res.StatusCode)

	buf := new(bytes.Buffer)
	_, _ = buf.ReadFrom(res.Body)
	cupaloy.SnapshotT(suite.T(), buf.String())
}

func TestHealthSuite(t *testing.T) {
	suite.Run(t, new(Suite))
}
