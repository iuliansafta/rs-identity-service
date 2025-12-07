# iulian.cloud - Rust & Axum Authentication Service

## Overview
This service provides authentication and authorization functionality for iulian.cloud applications. It supports various authentication methods such as email, and password.

## Features
- Email verification
- Password reset
- User registration
- User profile management


## Local Development
Use `.envrc.example` to understand what environment variables need to be set

1. Install direnv [direnv install](https://direnv.net/docs/installation.html)

```bash
cp .envrc.example .envrc
# make the changes to the .envrc file
direnv allow .
```

## Database migrations (sea-orm)
Manage our database schema.

Create a migration table
```sh 
sea-orm-cli migrate generate create_table_{my_table}
```

Run migration
```sh
# up
cd migration && cargo run -- up

# down
cd migration && cargo run -- down
```

### Generate models
```bash
sea-orm-cli generate entity --output-dir ./models/src --lib --entity-format dense --with-serde both

```
