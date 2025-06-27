install:
	brew install goose

db.migrate.create:
db.migrate.create:
	goose -dir db/migrations create $(name) sql

db.migrate.up:
db.migrate.up:
	goose -allow-missing -dir db/migrations postgres "postgres://postgres:postgres@localhost:5432/oldworld-test?connect_timeout=180&sslmode=disable" up

db.migrate.down:
db.migrate.down:
	goose -dir db/migrations postgres "postgres://postgres:postgres@localhost:5432/oldworld-test?connect_timeout=180&sslmode=disable" down

db.migrate.reset:
db.migrate.reset:
	goose -dir db/migrations postgres "postgres://postgres:postgres@localhost:5432/oldworld-test?connect_timeout=180&sslmode=disable" reset

db.migrate.validate:
db.migrate.validate:
	goose -dir=./db/migrations -v validate
