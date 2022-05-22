### PostgreSQL
Install PostgreSQL.
```
brew install postgresql
```

Start the database server.
```
pg_ctl -D /usr/local/var/postgres start
```

Create a user.
```
createuser --interactive --pwprompt
```

Create a database.
```
createdb rest_api_postgres
```

### Diesel
Set up Diesel.
```
diesel setup
```

Generate database migration files. The following command will generate two sql files. `up.sql` defines how database migration will be applied and `down.sql` defines how it will be reverted.
```
diesel migration generate add_users
```

Execute database migration.
```
diesel migration run
```