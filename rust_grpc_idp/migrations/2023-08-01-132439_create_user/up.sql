-- Your SQL goes here
CREATE TABLE users (
      user_id SERIAL PRIMARY KEY,
      username VARCHAR(255) UNIQUE NOT NULL,
      password_hash VARCHAR(255) NOT NULL,
      email VARCHAR(255) UNIQUE NOT NULL,
      fullname VARCHAR(255) NOT NULL,
      phone_number VARCHAR(255),
      created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
      profile_picture_url VARCHAR(2083),
      email_validated BOOLEAN DEFAULT FALSE
);