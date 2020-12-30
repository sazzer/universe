package users_test

import (
	"encoding/json"
	"testing"

	"github.com/sazzer/universe/service/internal/users"
	"github.com/stretchr/testify/assert"
)

func TestMarshalUserID(t *testing.T) {
	userID, err := users.ParseUserID("f783d2db-b0fc-4fb4-b89a-5beead0f582b")
	assert.NoError(t, err)

	encoded, err := json.Marshal(userID)
	assert.NoError(t, err)

	assert.Equal(t, "\"f783d2db-b0fc-4fb4-b89a-5beead0f582b\"", string(encoded))
}

func TestUnmarshalUserIDSuccess(t *testing.T) {
	t.Parallel()

	tests := []string{
		"\"f783d2db-b0fc-4fb4-b89a-5beead0f582b\"",
		"\"  f783d2db-b0fc-4fb4-b89a-5beead0f582b\"",
		"\"f783d2db-b0fc-4fb4-b89a-5beead0f582b  \"",
		"\"  f783d2db-b0fc-4fb4-b89a-5beead0f582b  \"",
	}

	for _, input := range tests {
		input := input
		t.Run(input, func(t *testing.T) {
			t.Parallel()

			var userID users.UserID
			err := json.Unmarshal([]byte(input), &userID)
			assert.NoError(t, err)

			assert.Equal(t, "f783d2db-b0fc-4fb4-b89a-5beead0f582b", userID.String())
		})
	}
}

func TestUnmarshalUserIDFailure(t *testing.T) {
	t.Parallel()

	tests := []string{
		"\"f783d2db-b0fc-4fb4-b89a-5beead0f582\"",
		"\"f783d2db-b0fc-4fb4-b89a-5beead0f582bc\"",
		"\"f783d2db-b0fc-4fb4-b89a-5beead0f582z\"",
		"\"  \"",
		"\"\"",
		"{}",
		"7",
		"false",
	}

	for _, input := range tests {
		input := input
		t.Run(input, func(t *testing.T) {
			t.Parallel()

			var userID users.UserID
			err := json.Unmarshal([]byte(input), &userID)
			assert.Error(t, err)
		})
	}
}

func TestParseUserIDSuccess(t *testing.T) {
	t.Parallel()

	tests := []string{
		"f783d2db-b0fc-4fb4-b89a-5beead0f582b",
		"  f783d2db-b0fc-4fb4-b89a-5beead0f582b",
		"f783d2db-b0fc-4fb4-b89a-5beead0f582b  ",
		"  f783d2db-b0fc-4fb4-b89a-5beead0f582b  ",
	}

	for _, input := range tests {
		input := input
		t.Run(input, func(t *testing.T) {
			t.Parallel()

			userID, err := users.ParseUserID(input)
			assert.NoError(t, err)

			assert.Equal(t, "f783d2db-b0fc-4fb4-b89a-5beead0f582b", userID.String())
		})
	}
}

func TestParseUserIDFailure(t *testing.T) {
	t.Parallel()

	tests := []string{
		"f783d2db-b0fc-4fb4-b89a-5beead0f582",
		"f783d2db-b0fc-4fb4-b89a-5beead0f582bc",
		"f783d2db-b0fc-4fb4-b89a-5beead0f582z",
		"  ",
		"",
	}

	for _, input := range tests {
		input := input
		t.Run(input, func(t *testing.T) {
			t.Parallel()

			_, err := users.ParseUserID(input)
			assert.Error(t, err)
		})
	}
}
