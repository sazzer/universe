package users

import (
	"encoding/json"
	"fmt"
	"strings"

	"github.com/google/uuid"
)

// The ID of the User.
type UserID struct {
	data uuid.UUID
}

// Generate a new User ID.
func NewUserID() UserID {
	return UserID{
		data: uuid.New(),
	}
}

// Parse a User ID out of the provided string.
func ParseUserID(input string) (UserID, error) {
	id, err := uuid.Parse(strings.TrimSpace(input))
	if err != nil {
		return UserID{}, fmt.Errorf("failed to parse User ID: %w", err)
	}

	return UserID{data: id}, nil
}

// Marshal a User ID as JSON.
func (u UserID) MarshalJSON() ([]byte, error) {
	return json.Marshal(u.data)
}

// Extract the User ID as a string.
func (u UserID) String() string {
	return u.data.String()
}
