package users

import (
	"fmt"

	"github.com/alexedwards/argon2id"
	"github.com/rs/zerolog/log"
)

// The password of a user.
type Password struct {
	hash string
}

// Create a Password struct wrapping the provided hash.
func NewPassword(hash string) Password {
	return Password{
		hash,
	}
}

// Hash a plaintext password to generate a secure version for storage.
func HashPassword(plaintext string) (Password, error) {
	hash, err := argon2id.CreateHash(plaintext, argon2id.DefaultParams)
	if err != nil {
		log.Error().Err(err).Msg("Failed to hash password")

		return Password{}, fmt.Errorf("failed to hash password: %w", err)
	}

	return Password{hash}, nil
}

// Compare a hashed password to a plaintext one to see if they are the same.
func (p Password) Equals(plaintext string) bool {
	result, err := argon2id.ComparePasswordAndHash(plaintext, p.hash)
	if err != nil {
		log.Error().Err(err).Msg("Failed to compare passwords")

		return false
	}

	return result
}
