#!/bin/bash

# This script is only intended to be used for development environment.

# Generate the Index sqlite database directory and file if it does not exist
mkdir -p ./storage/database

if ! [ -f "./storage/database/sqlite3.db" ]; then
    echo "Creating database 'sqlite3.db'"
    sqlite3 "./storage/database/sqlite3.db" "VACUUM;"
fi
