#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE TABLE Posts (
         id        SERIAL PRIMARY KEY,
         user_id   SERIAL NOT NULL,
         title     VARCHAR NOT NULL,
         body      TEXT    NOT NULL
         )
EOSQL
