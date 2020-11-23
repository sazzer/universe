CREATE TABLE USERS(
  user_id UUID PRIMARY KEY,
  version UUID NOT NULL,
  created TIMESTAMP WITH TIME ZONE NOT NULL,
  updated TIMESTAMP WITH TIME ZONE NOT NULL,
  username TEXT NULL UNIQUE,
  email TEXT NULL UNIQUE,
  display_name TEXT NOT NULL,
  authentications JSONB NOT NULL
);