# Rubble

a lightweight blog engine written by Rust.

**PS: And this project is still working in progress, so API of it are very unstable, including the content of the database migrations. I can promise that the elder version of it can be upgraded to the newest one. It means that the best way is installing a new one , instead of upgarding it.**

## Feature

Cause this project is also the tentative staff I try to write something in Rust, it would not include too many features.

- [*] Basic Content System without categories
- [ ] Multiple administrators supported
- [*] Administractor management panel
- [*] Article management panel
- [*] Draw supported
- [ ] Customized template
- [ ] RSS supported
- [ ] GraphQL API supported(maybe)

## Template 

Project rubble highly depends on tera, a fast and effective template engine in Rust, which means that you can write your own template with tera syntax.

There are files in template folder as follow, which are the template for each page:

- `admin` folder
  - `index.tera` index of admin panel
  - `login.tera` admin login page
- `index.tera` index of whole site
- `archives` template of single article page

Obviously you can learn how to write this template by the guide of official template folder, and how to use tera syntax in tera's official website.

## Deploy using Docker

you can easily use Docker to create your own rubble application. And the latest version of it and each tagged version would be built as docker images storing in Docker Hub automatically. So you can easily pull those images by using `docker pull rubble:latest`

Rubble uses PostgresQL as data storage, so before strating rubble application, you need to start your postgres service and link it to rubble.

Rubble image can accept some environment variable for setting up:

- `DATABASE_URL` url of postgresQL
- `SECRET_KEY` secret ke of rubble for creating secret cookies

### Docker Stack

But we recommend to deploy rubble with Docker Swarm or Kubenetes. here is a simple file to create a whole rubble application with postgresQL`docker-compose.yml` :


```yml
version: "3"
services:
  rubble:
    image: kilerd/rubble
    environment:
      DATABASE_URL: postgres://root:password@postgres/rubble
      ROCKET_SECRET_KEY: 7azo1R1AtDzvj2bwv7Qj949xrvBnTzF+EEwvBSDsLWs=
    depends_on:
      - postgres
    networks:
      - backend

  postgres:
    image: postgres:9-alpine
    restart: always
    environment:
      POSTGRES_USER: root
      POSTGRES_PASSWORD: password
      POSTGRES_DB: rubble
    networks:
      - backend


networks:
  backend:
```

