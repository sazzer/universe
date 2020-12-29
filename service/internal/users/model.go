package users

import (
	"fmt"
	"time"

	"github.com/alexedwards/argon2id"
	"github.com/gofrs/uuid"
	"github.com/rs/zerolog/log"
)

// The ID of the User.
type UserID uuid.UUID

// The email address of a user.
type Email string

// The username of a user.
type Username string

// The password of a user.
type Password string

// Hash a plaintext password to generate a secure version for storage.
func HashPassword(plaintext string) (Password, error) {
	hash, err := argon2id.CreateHash(plaintext, argon2id.DefaultParams)
	if err != nil {
		log.Error().Err(err).Msg("Failed to hash password")

		return "", fmt.Errorf("failed to hash password: %w", err)
	}

	return Password(hash), nil
}

// Compare a hashed password to a plaintext one to see if they are the same.
func (p Password) Equals(plaintext string) bool {
	result, err := argon2id.ComparePasswordAndHash(plaintext, string(p))
	if err != nil {
		log.Error().Err(err).Msg("Failed to compare passwords")

		return false
	}

	return result
}

// The data represnting a user.
type UserData struct {
	Email       Email
	Username    Username
	DisplayName string
	Password    Password
}

// Model representing a saved user.
type UserModel struct {
	UserData
	ID      UserID
	Version string
	Created time.Time
	Updated time.Time
}
