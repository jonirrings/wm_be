#!/bin/bash

# This script is only intended to be used for development environment.

# Generate the Index sqlite database directory and file if it does not exist
mkdir -p ./storage/database

if ! [ -f "./storage/database/wm.sqlite" ]; then
    echo "Creating database 'wm.sqlite'"
    sqlite3 "./storage/database/wm.sqlite" "VACUUM;"
fi
