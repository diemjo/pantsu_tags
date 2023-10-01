DO $$
BEGIN
    IF NOT EXISTS(
        SELECT schema_name
        FROM information_schema.schemata
        WHERE schema_name = 'migration'
    )
    THEN
        CREATE SCHEMA migration;
        CREATE TABLE IF NOT EXISTS migration.migrations (
            version varchar NOT NULL PRIMARY KEY,
            hash char(16) NOT NULL,
            sql varchar NOT NULL
        );
    END IF;
END
$$
