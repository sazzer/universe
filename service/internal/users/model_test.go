package users_test

import (
	"testing"

	"github.com/sazzer/universe/service/internal/users"
	"github.com/stretchr/testify/assert"
)

func TestHashPassword(t *testing.T) {
	t.Parallel()

	tests := []string{
		"password",
		"Password",
		"Password123",
		"!@#$%^&*()_+",
		"كلمه السر",
	}

	for _, input := range tests {
		input := input
		t.Run(input, func(t *testing.T) {
			t.Parallel()

			password, err := users.HashPassword(input)

			assert.NoError(t, err)
			assert.NotEqual(t, password, input)

			match := password.Equals(input)
			assert.True(t, match)
		})
	}
}

func TestPasswordsDiffer(t *testing.T) {
	password, err := users.HashPassword("password")

	assert.NoError(t, err)

	t.Parallel()

	tests := []string{
		"Password",
		"passwords",
		"passwor",
		"",
	}

	for _, input := range tests {
		input := input
		t.Run(input, func(t *testing.T) {
			t.Parallel()

			match := password.Equals(input)
			assert.False(t, match)
		})
	}
}
