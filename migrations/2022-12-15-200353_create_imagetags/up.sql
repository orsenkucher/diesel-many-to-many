CREATE TABLE image_tags (
    id UUID PRIMARY KEY,
    image_id UUID references images(id) NOT NULL,
    tag_id UUID references tags(id) NOT NULL
)
