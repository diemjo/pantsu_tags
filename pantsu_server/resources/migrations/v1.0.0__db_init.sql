/*
 example users entry (
   id:        0,
   user_id:   'gusti',
   user_name: 'Gustavo Alonso',
 )
 */
CREATE TABLE users (
    id SERIAL NOT NULL,
    user_id VARCHAR(64) NOT NULL,
    user_name VARCHAR(64) NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT distinct_user_id UNIQUE (user_id)
);

-- TODO: who shares with whom?
-- TODO: guest flag?

CREATE TYPE image_format_enum AS ENUM ('PNG', 'JPEG');

/*
 example images entry (
   id:                 0,
   id_hash:            '\xfff0e5fb34d6f989',
   perceptual_hash:    '\x0403fc9df0fc0fc0fc0f98f88f1090b8abfb',
   image_format:       'PNG',
   width:              1920,
   height:             1080,
   created_at:         '2023-10-30 11:00:00+1',
   modified_at:        '2023-11-04 11:00:00+1',
 )
 */
CREATE TABLE images (
    id SERIAL NOT NULL,
    id_hash bytea NOT NULL,
    perceptual_hash bytea NOT NULL,
    image_format image_format_enum NOT NULL,
    width INT NOT NULL,
    height INT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    modified_at TIMESTAMP WITH TIME ZONE NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT id_hash_size CHECK ( length(id_hash) = 8 ),
    CONSTRAINT perceptual_hash_size CHECK ( length(perceptual_hash) = 18 ),
    CONSTRAINT positive_width CHECK ( width > 0 ),
    CONSTRAINT positive_height CHECK ( height > 0 ),
    CONSTRAINT distinct_id_hash UNIQUE (id_hash),
    CONSTRAINT modification_not_before_creation CHECK ( created_at <= images.modified_at )
);

/*
 example source_status entry (
   id:                   0,
   image_id:             1 -> 'fff0e5fb34d6f989',
   site:                 'gelbooru',
   url:                  'https://gelbooru.com/index.php?page=post&s=view&id=9071108',
   author:               1 -> 'gusti',
   last_checked_at:      '2023-11-04 11:00:00+1',
   last_checked_tags_at: '2023-11-04 11:00:00+1',
 )
 */
CREATE TABLE sources (
    id SERIAL NOT NULL,
    image_id INT NOT NULL,
    site VARCHAR(64) NOT NULL,
    url VARCHAR(1024),
    author INT NOT NULL,
    last_checked_at TIMESTAMP WITH TIME ZONE NOT NULL,
    last_checked_tags_at TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (id),
    CONSTRAINT distinct_image_id_site UNIQUE (image_id, site)
);

/*
 example user_images entry (
   id:       0,
   user_id:  1 -> 'gusti',
   image_id: 2 -> 'fff0e5fb34d6f989',
 )
 */
CREATE TABLE user_images (
    id SERIAL NOT NULL,
    user_id INT NOT NULL,
    image_id INT NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users(id),
    CONSTRAINT fk_image_id FOREIGN KEY (image_id) REFERENCES images(id)
);

/*
 example tags entry (
   id:           0,
   category: 'general',
   name:     'pantsu',
   author:   1 -> 'gusti',
 )
 */
CREATE TABLE tags (
    id SERIAL NOT NULL,
    category VARCHAR(64) NOT NULL,
    name VARCHAR(256) NOT NULL,
    author INT NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT distinct_category_name UNIQUE (category, name),
    CONSTRAINT fk_tag_author FOREIGN KEY (author) REFERENCES users(id)
);

-- TODO: think about whitelist/blacklist/private tags/categories

/*
 example image_tags entry (
   id:         0,
   image_id:   1 -> 'fff0e5fb34d6f989',
   tag_id:     2 -> 'general:pantsu',
   tag_author: 3 -> 'bertel',
   added_at:   '2023-10-30 11:00:00+1',
 )
 */
CREATE TABLE image_tags (
    id SERIAL NOT NULL,
    image_id INT NOT NULL,
    tag_id INT NOT NULL,
    tag_author INT NOT NULL,
    added_at TIMESTAMP WITH TIME ZONE NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT fk_tag_id FOREIGN KEY (tag_id) REFERENCES tags(id),
    CONSTRAINT fk_image_id FOREIGN KEY (image_id) REFERENCES images(id),
    CONSTRAINT fk_tag_author FOREIGN KEY (tag_author) REFERENCES users(id),
    CONSTRAINT distinct_image_id_tag_id UNIQUE (image_id, tag_id)
);
