pub const INSERT_USER: &str = "
    INSERT INTO users (uuid, email, hashed_password)
    VALUES ($1, $2, $3)
    RETURNING jsonb_build_object(
        'uuid', uuid,
        'email', email,
        'hashed_password', hashed_password,
        'created_at', created_at,
        'updated_at', updated_at
    ) AS user_entity;
";

pub const GET_USER_BY_EMAIL: &str = "
    SELECT jsonb_build_object(
        'uuid', uuid,
        'email', email,
        'hashed_password', hashed_password,
        'created_at', created_at,
        'updated_at', updated_at
    ) AS user_entity
    FROM users
    WHERE email = $1;
";

pub const GET_USER_BY_UUID: &str = "
    SELECT jsonb_build_object(
        'uuid', uuid,
        'email', email,
        'hashed_password', hashed_password,
        'created_at', created_at,
        'updated_at', updated_at
    ) AS user_entity
    FROM users
    WHERE uuid = $1;
";

pub const INSERT_USER_CHOICE: &str = "
    INSERT INTO user_choices (user_uuid, recommendation_type)
    VALUES ($1, $2)
";

pub const GET_LAST_USER_SIMILAR_CHOICE: &str = "
    SELECT jsonb_build_object(
        'recommendation_type', recommendation_type,
        'inserted_at', inserted_at,
        'user_uuid', user_uuid
    ) AS user_choice
    FROM user_choices
    WHERE user_uuid = $1 AND recommendation_type = $2
    ORDER BY inserted_at DESC
    LIMIT 1
";
