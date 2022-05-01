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