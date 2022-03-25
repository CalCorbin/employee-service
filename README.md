# Employee Service

Employee Service is a repo that houses the employee service API I built in Rust. This repo is based off this tutorial: https://blog.logrocket.com/create-backend-api-with-rust-postgres/#setting-up-postgres-connection

## Creating a Postgres Database

1. Run `brew install postgresql`
2. Open the psql shell.
3. Connect to the DB, aka press enter 5 times.
4. Run `CREATE DATABASE employee_service;`
5. Run `\l` to confirm that the database was created.
6. Connect to the database by running `\c employee_service`.
7. After confirming that the database was created, create a user.
8. Back in the psql shell, run `create user <your username> with encrypted password '<whatever password>';`.
9. Grant privileges to the database with `grant all privileges on database employee_service to <username>;`

Useful reference for setting up the database: https://medium.com/coding-blocks/creating-user-database-and-adding-access-on-postgresql-8bfcd2f4a91e
