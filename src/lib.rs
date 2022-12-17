use std::env;

use diesel::prelude::*;
use dotenvy::dotenv;
use models::{Image, ImageTag, NewImage, NewImageTag, NewTag, Tag};

use crate::models::WithId;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("error connecting to {db_url}"))
}

fn reset_db(conn: &mut PgConnection) {
    use self::schema::image_tags::dsl::*;
    use self::schema::images::dsl::*;
    use self::schema::tags::dsl::*;

    diesel::delete(image_tags)
        .execute(conn)
        .expect("could no delete image_tags associations");

    diesel::delete(tags)
        .execute(conn)
        .expect("could not delete tags");

    diesel::delete(images)
        .execute(conn)
        .expect("could not delete images");
}

fn insert_test_data(conn: &mut PgConnection) -> (Image, Image) {
    use self::schema::image_tags;
    use self::schema::images;
    use self::schema::tags;

    let new_img1 = NewImage { url: "img.jpg" };
    let new_img2 = NewImage { url: "img.png" };

    let img1: Image = diesel::insert_into(images::table)
        .values(new_img1.with_id())
        .get_result(conn)
        .expect("error saving img1");
    let img2: Image = diesel::insert_into(images::table)
        .values(new_img2.with_id())
        .get_result(conn)
        .expect("error saving img2");

    let cat_tag = NewTag { label: "cat" };
    let cute_tag = NewTag { label: "cute" };
    let tag1: Tag = diesel::insert_into(tags::table)
        .values(cat_tag.with_id())
        .get_result(conn)
        .expect("error saving cat tag");
    let tag2: Tag = diesel::insert_into(tags::table)
        .values(cute_tag.with_id())
        .get_result(conn)
        .expect("error saving cute tag");

    // Associate images with tags
    let img1_tag1 = NewImageTag {
        image_id: img1.id,
        tag_id: tag1.id,
    };
    let img2_tag2 = NewImageTag {
        image_id: img2.id,
        tag_id: tag2.id,
    };
    diesel::insert_into(image_tags::table)
        .values(img1_tag1.with_id())
        .execute(conn)
        .expect("error associating img1 with tag1");

    diesel::insert_into(image_tags::table)
        .values(img2_tag2.with_id())
        .execute(conn)
        .expect("error associating img2 with tag2");

    (img1, img2)
}

fn get_tags_for_image(img: &Image, conn: &mut PgConnection) -> Vec<Tag> {
    use self::schema::image_tags;
    use self::schema::tags;

    let image_tag_ids = ImageTag::belonging_to(img).select(image_tags::tag_id);
    tags::table
        .filter(tags::id.eq_any(image_tag_ids))
        .load::<Tag>(conn)
        .expect("could not load tags")
}

pub fn list_tags() {
    let conn = &mut establish_connection();
    println!("reset db");
    reset_db(conn);
    println!("insert image data");
    let (img1, img2) = insert_test_data(conn);

    let result = get_tags_for_image(&img1, conn);
    println!("Image 1 has {} tags.", result.len());
    for t in result {
        println!("{}: {}", t.id, t.label);
    }

    let result = get_tags_for_image(&img2, conn);
    println!("Image 2 has {} tags.", result.len());
    for t in result {
        println!("{}: {}", t.id, t.label);
    }
}
