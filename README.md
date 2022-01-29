# Wabper

_Wabper_ aims to be a simple and customisable pastebin-like app written in Rust.

### Docker-compose example:

```yaml
version: "3"
networks:
  db:
    external: false

services:
  wabper:
    env_file: ".env"
    image: "woomy4680/wabper:latest"
    ports:
      - "8080:8080"
    networks:
      - "db"
  db:
    image: "postgres:14.1"
    environment:
      - POSTGRES_USER=postgres
      - POSTGRES_DB=wabper
      - POSTGRES_PASSWORD=password
    networks:
      - "db"
    volumes:
      - "./wabper-data:/var/lib/postgresql/data"
```
