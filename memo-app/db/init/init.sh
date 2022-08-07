#!/bin/bash

mysql -u root -proot memo-app < "/docker-entrypoint-initdb.d/init-schema.sql"
