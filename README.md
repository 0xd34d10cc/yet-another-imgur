# Setup
- Set DATABASE_URL environment variable (e.g. postgresql://localhost/mydb)
- Setup database:
```
$ cargo install diesel_cli
$ diesel setup
```
- Run
```
$ cargo run
```

# Valid requests
- POST /images/upload with Content-Type: multipart/form-data where each item is image
- POST /images/upload with Content-Type: application/json with body matching Request struct
- GET /images/\<id\>/preview
