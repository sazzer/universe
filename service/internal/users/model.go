package users

import "time"

// The data representing a user.
type UserData struct {
	Username    Username
	Email       Email
	DisplayName string
	Password    Password
}

// Model representation of a persisted user.
type UserModel struct {
	UserData
	ID      UserID
	Version string
	Created time.Time
	Updated time.Time
}
