// @generated automatically by Diesel CLI.

diesel::table! {
    memos (id) {
        id -> Integer,
        memo -> Nullable<Text>,
        created_at -> Datetime,
    }
}
