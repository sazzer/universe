package users

import (
	"encoding/json"
	"errors"
	"strings"
)

// ErrBlankEmail is an error indicating that an email address was blank.
var ErrBlankEmail = errors.New("email address was blank")

// The email address of the User.
type Email struct {
	data string
}

// Parse an Email out of the provided string.
func ParseEmail(input string) (Email, error) {
	value := strings.TrimSpace(input)
	if value == "" {
		return Email{}, ErrBlankEmail
	}

	return Email{data: value}, nil
}

// Marshal an email address as JSON.
func (e Email) MarshalJSON() ([]byte, error) {
	return json.Marshal(e.data)
}

// Extract the email address as a string.
func (e Email) String() string {
	return e.data
}
