package users

import (
	"encoding/json"
	"errors"
	"strings"
)

// ErrBlankUsername is an error indicating that a username was blank.
var ErrBlankUsername = errors.New("username was blank")

// The username of the User.
type Username struct {
	data string
}

// Parse a username out of the provided string.
func ParseUsername(input string) (Username, error) {
	value := strings.TrimSpace(input)
	if value == "" {
		return Username{}, ErrBlankUsername
	}

	return Username{data: value}, nil
}

// Marshal a username as JSON.
func (e Username) MarshalJSON() ([]byte, error) {
	return json.Marshal(e.data)
}

// Extract the username as a string.
func (e Username) String() string {
	return e.data
}
