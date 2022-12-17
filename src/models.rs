use diesel::prelude::*;
use uuid::Uuid;

use super::schema::*;

#[derive(Identifiable, Queryable, PartialEq, Clone, Debug)]
pub struct Image {
    pub id: Uuid,
    pub url: String,
}

#[derive(Insertable)]
#[diesel(table_name = images)]
pub struct NewImage<'a> {
    pub url: &'a str,
}

#[derive(Identifiable, Queryable, PartialEq, Clone, Debug)]
pub struct Tag {
    pub id: Uuid,
    pub label: String,
}

#[derive(Insertable)]
#[diesel(table_name = tags)]
pub struct NewTag<'a> {
    pub label: &'a str,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Clone, Debug)]
#[diesel(belongs_to(Image))]
#[diesel(belongs_to(Tag))]
#[diesel(table_name = image_tags)]
pub struct ImageTag {
    pub id: Uuid,
    pub image_id: Uuid,
    pub tag_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = image_tags)]
pub struct NewImageTag {
    pub image_id: Uuid,
    pub tag_id: Uuid,
}

pub fn id_for<Id>(id: Id) -> diesel::dsl::Eq<Id, uuid::Uuid>
where
    Id: diesel::Expression<SqlType = diesel::sql_types::Uuid>,
{
    id.eq(Uuid::new_v4())
}

pub trait WithId {
    type Id: diesel::Expression<SqlType = diesel::sql_types::Uuid>;

    fn id() -> Self::Id;

    fn with_id(self) -> (Self, diesel::dsl::Eq<Self::Id, Uuid>)
    where
        Self: Sized,
    {
        (self, id_for(Self::id()))
    }
}

impl WithId for NewImage<'_> {
    type Id = images::id;

    fn id() -> Self::Id {
        images::id
    }
}

impl WithId for NewTag<'_> {
    type Id = tags::id;

    fn id() -> Self::Id {
        tags::id
    }
}

impl WithId for NewImageTag {
    type Id = image_tags::id;

    fn id() -> Self::Id {
        image_tags::id
    }
}
