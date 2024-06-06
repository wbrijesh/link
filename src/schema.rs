// @generated automatically by Diesel CLI.

diesel::table! {
    links_backup (id) {
        id -> Integer,
        url -> Text,
        slug -> Text,
    }
}
