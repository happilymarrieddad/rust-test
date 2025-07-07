install:
	cargo install sqlx-cli --no-default-features --features native-tls,postgres

test:
	cargo test

db.init:
	sqlx database create

db.drop:
	sqlx database drop

db.add:
	sqlx migrate add -r $(name)

db.up:
	sqlx migrate run

db.reset:
	sqlx migrate revert
	make db.up
