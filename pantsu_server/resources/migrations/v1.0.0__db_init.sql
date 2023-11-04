CREATE TABLE users (
    user_id VARCHAR NOT NULL,
    user_name VARCHAR NOT NULL,
    PRIMARY KEY (user_id)
);

CREATE TYPE image_format_enum AS ENUM ('PNG', 'JPEG');

CREATE TABLE images (
    id_hash bytea NOT NULL CHECK ( length(id_hash) = 8 ),
    perceptual_hash bytea NOT NULL CHECK ( length(perceptual_hash) = 18 ),
    image_format image_format_enum NOT NULL,
    width INT NOT NULL CHECK ( width > 0 ),
    height INT NOT NULL CHECK ( height > 0 ),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    modified_at TIMESTAMP WITH TIME ZONE NOT NULL,
    PRIMARY KEY (id_hash),
    CONSTRAINT modification_not_before_creation CHECK ( created_at <= images.modified_at )
);
