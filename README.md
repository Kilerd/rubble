# Rubble

<img align="right" width="128" height="128" src="/rubble.png">

a lightweight blog engine written by Rust.

## Feature

Cause this project is also the tentative staff I try to write something in Rust, it would not include too many features.

- [x] Basic Content System without categories
- [ ] Multiple administrators supported
- [x] Administractor management panel
- [x] Article management panel
- [x] Draw supported
- [x] Customized template
- [x] RSS supported

## Template 

Project rubble highly depends on tera, a fast and effective template engine in Rust, which means that you can write your own template with tera syntax.

There are files in template folder as follow, which are the template for each page:

- `admin` folder
  - `panel.html` dashboard of admin panel
  - `login.html` admin login page
- `homepage.html` index of whole site
- `archives.html` template of single article page

Obviously you can learn how to write this template by the guide of official template folder, and how to use tera syntax in tera's official website.
## How to use it

After deploying rubble to your host, the first thing you need to do is login to the admin panel with url `http://yourdomain.com/admin`. And the default admin user and password is as follow:
 - Username: `admin`
 - Password: `password`

after logging in, please modify the default password of admin. Then you can enjoy the whole project system.

## Deploy using Docker

you can easily use Docker to create your own rubble application. And the latest version of it and each tagged version would be built as docker images storing in Docker Hub automatically. So you can easily pull those images by using `docker pull kilerd/rubble:latest`

Rubble uses PostgresQL as data storage, so before strating rubble application, you need to start your postgres service and link it to rubble.

Rubble image can accept some environment variable for setting up:

- `DATABASE_URL` url of postgresQL

### Docker Stack

But we recommend to deploy rubble with Docker Swarm or Kubenetes. here is a simple file to create a whole rubble application with postgresQL`docker-compose.yml` :


```yml
version: "3"
services:
  rubble:
    image: kilerd/rubble:latest
    environment:
      DATABASE_URL: postgres://root:password@postgres/rubble
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