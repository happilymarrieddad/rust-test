-- +goose Up
CREATE SCHEMA IF NOT EXISTS tiger;
CREATE SCHEMA IF NOT EXISTS tiger_data;
CREATE SCHEMA IF NOT EXISTS topology;COMMENT ON SCHEMA topology IS 'PostGIS Topology schema';
CREATE EXTENSION IF NOT EXISTS address_standardizer;COMMENT ON EXTENSION address_standardizer IS 'Used to parse an address into constituent elements. Generally used to support geocoding address normalization step.';
CREATE EXTENSION IF NOT EXISTS address_standardizer_data_us;COMMENT ON EXTENSION address_standardizer_data_us IS 'Address Standardizer US dataset example';
CREATE EXTENSION IF NOT EXISTS btree_gist;COMMENT ON EXTENSION btree_gist IS 'support for indexing common datatypes in GiST';
CREATE EXTENSION IF NOT EXISTS citext;COMMENT ON EXTENSION citext IS 'data type for case-insensitive character strings';
CREATE EXTENSION IF NOT EXISTS fuzzystrmatch;COMMENT ON EXTENSION fuzzystrmatch IS 'determine similarities and distance between strings';
CREATE EXTENSION IF NOT EXISTS postgis;COMMENT ON EXTENSION postgis IS 'PostGIS geometry, geography, and raster spatial types and functions';
CREATE EXTENSION IF NOT EXISTS postgis_tiger_geocoder WITH SCHEMA tiger;COMMENT ON EXTENSION postgis_tiger_geocoder IS 'PostGIS tiger geocoder and reverse geocoder';
CREATE EXTENSION IF NOT EXISTS postgis_topology WITH SCHEMA topology;COMMENT ON EXTENSION postgis_topology IS 'PostGIS topology spatial types and functions';
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";COMMENT ON EXTENSION "uuid-ossp" IS 'generate universally unique identifiers (UUIDs)';

CREATE TABLE IF NOT EXISTS users(
    id              BIGSERIAL PRIMARY KEY NOT NULL
    ,first_name     CITEXT NOT NULL
    ,last_name      CITEXT NOT NULL
    ,email          CITEXT NOT NULL
    ,password       CITEXT NOT NULL
    ,created_at     TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
    ,updated_at     TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
    ,active         BOOLEAN DEFAULT TRUE
    ,deleted_at     TIMESTAMP WITH TIME ZONE
    ,UNIQUE(email)
);

-- +goose Down
DROP TABLE IF EXISTS users;
