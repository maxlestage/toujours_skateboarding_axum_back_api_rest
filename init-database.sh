#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" <<-EOSQL
  CREATE DATABASE toujours_skateboarding_axum_db;
  \c toujours_skateboarding_axum_db
  create extension if not exists "uuid-ossp";
EOSQL