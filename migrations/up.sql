-- creators
CREATE TABLE creators (
    id SERIAL PRIMARY KEY,
    username VARCHAR(100) UNIQUE NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    email VARCHAR(150) UNIQUE NOT NULL,
    bio TEXT,
    profile_image TEXT,
    wallet_address VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- tips
CREATE TABLE tips (
    id SERIAL PRIMARY KEY,
    creator_id INT REFERENCES creators(id) ON DELETE CASCADE,
    tip_amount DECIMAL(10,2) NOT NULL,
    tipper_wallet VARCHAR(255),
    message TEXT,
    transaction_signature VARCHAR(255) UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- activity_logs
CREATE TABLE activity_logs (
    id SERIAL PRIMARY KEY,
    creator_id INT REFERENCES creators(id) ON DELETE CASCADE,
    action_type VARCHAR(100),
    metadata JSONB,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
