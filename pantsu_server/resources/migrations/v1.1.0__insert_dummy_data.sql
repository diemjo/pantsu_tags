INSERT INTO users (id, user_id, user_name)
VALUES
    (0, 'mizugi', 'mizugi saikou'),
    (1, 'gusti', 'Gustavo Alonso'),
    (2, 'gelbooru', 'gelbooru.com website');

INSERT INTO images (id, id_hash, perceptual_hash, image_format, width, height, created_at, modified_at)
VALUES
    (0, '\xd1fdc27f6d958c68'::bytea, '\xe97e1701321f007fd3a03c17ccf8e79e7803'::bytea, 'PNG', 1, 1, '2023-10-30 11:00:00+1', '2023-10-30 12:00:00+1'),
    (1, '\xe17c331e907dad95'::bytea, '\x00f41f8efccfe6b003007f87f83fe3802f0e'::bytea, 'JPEG', 50, 200, '2023-10-30 12:00:00+1', '2023-10-30 13:00:00+1'),
    (2, '\x0a5b903e5167da5a'::bytea, '\xf03e03f25f01e07e0fc0fc1f80f82fa2f80f'::bytea, 'PNG', 300, 200, '2023-10-30 13:00:00+1', '2023-10-30 14:00:00+1');

INSERT INTO user_images (user_id, image_id)
VALUES (0, 0), (0, 1), (0, 2), (1, 1), (1, 2);

INSERT INTO sources(id, image_id, site, url, author, last_checked_at, last_checked_tags_at)
VALUES
    (0, 0, 'gelbooru', 'https://gelbooru.com/index.php?page=post&s=view&id=9071108', 2, '2023-10-30 11:00:00+1', '2023-10-30 12:00:00+1'),
    (1, 1, 'gelbooru', NULL, 2, '2023-10-30 11:00:00+1', '2023-10-30 12:00:00+1'),
    (2, 0, 'pixiv', 'https://www.pixiv.net/en/artworks/108189434', 0, '2023-10-30 11:00:00+1', '2023-10-30 12:00:00+1');

INSERT INTO tags (id, category, name, author)
VALUES
    (0, 'general', 'pantsu', 2),
    (1, 'general', 'wakamezake', 0),
    (2, 'character', 'megumin', 2),
    (3, 'artist', 'mafuyu', 2);

INSERT INTO image_tags (id, image_id, tag_id, tag_author, added_at)
VALUES
    (0, 0, 0, 2, '2023-10-30 12:00:00+1'),
    (1, 0, 1, 0, '2023-10-30 13:00:00+1'),
    (2, 0, 2, 2, '2023-10-30 14:00:00+1'),
    (3, 1, 0, 2, '2023-10-30 15:00:00+1'),
    (4, 1, 3, 2, '2023-10-30 16:00:00+1')
