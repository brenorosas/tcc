CREATE TABLE user_choices (
    user_uuid UUID REFERENCES users(uuid),
    recommendation_type TEXT NOT NULL,
    inserted_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX user_choices_user_uuid_recommendation_type_inserted_at_idx ON user_choices(user_uuid, recommendation_type, inserted_at);