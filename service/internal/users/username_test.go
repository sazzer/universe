package users_test

import (
	"encoding/json"
	"testing"

	"github.com/sazzer/universe/service/internal/users"
	"github.com/stretchr/testify/assert"
)

func TestMarshalUsername(t *testing.T) {
	Username, err := users.ParseUsername("testuser")
	assert.NoError(t, err)

	encoded, err := json.Marshal(Username)
	assert.NoError(t, err)

	assert.Equal(t, "\"testuser\"", string(encoded))
}

func TestParseUsernameSuccess(t *testing.T) {
	t.Parallel()

	tests := []string{
		"testuser",
		"  testuser",
		"testuser  ",
		"  testuser  ",
	}

	for _, input := range tests {
		input := input
		t.Run(input, func(t *testing.T) {
			t.Parallel()

			Username, err := users.ParseUsername(input)
			assert.NoError(t, err)

			assert.Equal(t, "testuser", Username.String())
		})
	}
}

func TestParseUsernameFailure(t *testing.T) {
	t.Parallel()

	tests := []string{
		"  ",
		"",
	}

	for _, input := range tests {
		input := input
		t.Run(input, func(t *testing.T) {
			t.Parallel()

			_, err := users.ParseUsername(input)
			assert.Equal(t, users.ErrBlankUsername, err)
		})
	}
}
