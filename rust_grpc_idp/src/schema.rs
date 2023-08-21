// @generated automatically by Diesel CLI.

diesel::table! {
    auth_keys (code) {
        #[max_length = 255]
        code -> Varchar,
    }
}

diesel::table! {
    sessions (session_id) {
        #[max_length = 255]
        session_id -> Varchar,
        user_id -> Int4,
        created_at -> Nullable<Timestamp>,
        last_accessed -> Nullable<Timestamp>,
        expires_at -> Nullable<Timestamp>,
        ip_address -> Nullable<Text>,
        user_agent -> Nullable<Text>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        fullname -> Varchar,
        #[max_length = 255]
        phone_number -> Nullable<Varchar>,
        created_at -> Nullable<Timestamptz>,
        #[max_length = 2083]
        profile_picture_url -> Nullable<Varchar>,
        email_validated -> Nullable<Bool>,
        phone_number_validated -> Nullable<Bool>,
    }
}

diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    auth_keys,
    sessions,
    users,
);
