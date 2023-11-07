CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    "users" (
                id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
                name VARCHAR(100) NOT NULL,
                email VARCHAR(255) NOT NULL UNIQUE,
                password VARCHAR(100) NOT NULL,
                role VARCHAR(50) NOT NULL DEFAULT 'student',
                class INTEGER NOT NULL CHECK (class >= 1 AND class <= 12),
                created_at TIMESTAMP
                       WITH
                           TIME ZONE DEFAULT NOW(),
                updated_at TIMESTAMP
                       WITH
                           TIME ZONE DEFAULT NOW()
);