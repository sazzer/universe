package users_test

import (
	"encoding/json"
	"testing"

	"github.com/sazzer/universe/service/internal/users"
	"github.com/stretchr/testify/assert"
)

func TestMarshalEmail(t *testing.T) {
	Email, err := users.ParseEmail("testuser@example.com")
	assert.NoError(t, err)

	encoded, err := json.Marshal(Email)
	assert.NoError(t, err)

	assert.Equal(t, "\"testuser@example.com\"", string(encoded))
}

func TestParseEmailSuccess(t *testing.T) {
	t.Parallel()

	tests := []string{
		"testuser@example.com",
		"  testuser@example.com",
		"testuser@example.com  ",
		"  testuser@example.com  ",
	}

	for _, input := range tests {
		input := input
		t.Run(input, func(t *testing.T) {
			t.Parallel()

			Email, err := users.ParseEmail(input)
			assert.NoError(t, err)

			assert.Equal(t, "testuser@example.com", Email.String())
		})
	}
}

func TestParseEmailFailure(t *testing.T) {
	t.Parallel()

	tests := []string{
		"  ",
		"",
	}

	for _, input := range tests {
		input := input
		t.Run(input, func(t *testing.T) {
			t.Parallel()

			_, err := users.ParseEmail(input)
			assert.Equal(t, users.ErrBlankEmail, err)
		})
	}
}
