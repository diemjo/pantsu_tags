DO $$
BEGIN
    IF NOT EXISTS(
        SELECT schema_name
        FROM information_schema.schemata
        WHERE schema_name = 'migration'
    )
    THEN
        CREATE SCHEMA migration;
    END IF;
    IF NOT EXISTS(
        SELECT information_schema.tables.table_name
        FROM information_schema.tables
        WHERE table_schema = 'migration'
          AND table_name = 'migrations'
    )
    THEN
        CREATE TABLE migration.migrations (
            id SERIAL PRIMARY KEY,
            version varchar NOT NULL,
            description varchar NOT NULL,
            hash char(16) NOT NULL,
            sql varchar NOT NULL,
            UNIQUE(version)
        );
    END IF;
END
$$
