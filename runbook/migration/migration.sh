#!/bin/zsh

# at root folder of project.
cd ..
# create migration file
sea-orm-cli migrate generate create_user
# do the migration
DATABASE_URL="mysql://backend_user:backend_password@127.0.0.1:13306/komorebi" sea-orm-cli migrate refresh
