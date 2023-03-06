#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

    CREATE TABLE Users (
             id        UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
             name      VARCHAR(100) NOT NULL,
             email     VARCHAR(255) NOT NULL UNIQUE,
             password  VARCHAR(100) NOT NULL
             );

    CREATE TABLE Posts (
         id        UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
         user_id   UUID NOT NULL,
         title     VARCHAR NOT NULL,
         body      TEXT    NOT NULL
         );
EOSQL
