# Cr8s
=====================

## Start dockerfile
```bash
docker-compose up -d --build
```

## Setup database and create migrations folder
```bash
docker-compose exec app diesel setup
```

## Diesel commands
```bash
docker-compose exec app diesel migration list
```

```bash
docker-compose exec app diesel migration generate create_rustaceans
docker-compose exec app diesel migration generate create_crates
```

```bash
docker-compose exec app diesel migration run
```

```bash
docker-compose exec app diesel migration revert
docker-compose exec app diesel migration revert
docker-compose exec app diesel migration run
```
