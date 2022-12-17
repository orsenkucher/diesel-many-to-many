// @generated automatically by Diesel CLI.

diesel::table! {
    image_tags (id) {
        id -> Uuid,
        image_id -> Uuid,
        tag_id -> Uuid,
    }
}

diesel::table! {
    images (id) {
        id -> Uuid,
        url -> Varchar,
    }
}

diesel::table! {
    tags (id) {
        id -> Uuid,
        label -> Varchar,
    }
}

diesel::joinable!(image_tags -> images (image_id));
diesel::joinable!(image_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(image_tags, images, tags,);
