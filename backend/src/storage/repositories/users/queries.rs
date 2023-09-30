pub const INSERT_USER: &str = "
    INSERT INTO users (uuid, email, encrypted_password)
    VALUES ($1, $2, $3)
    RETURNING jsonb_build_object(
        'uuid', uuid,
        'email', email,
        'encrypted_password', encrypted_password,
        'created_at', created_at,
        'updated_at', updated_at
    ) AS user_entity;
";

pub const GET_USER_BY_EMAIL: &str = "
    SELECT jsonb_build_object(
        'uuid', uuid,
        'email', email,
        'encrypted_password', encrypted_password,
        'created_at', created_at,
        'updated_at', updated_at
    ) AS user_entity
    FROM users
    WHERE email = $1;
";
