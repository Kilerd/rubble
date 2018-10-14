-- Your SQL goes here
CREATE TABLE users (
  "id" serial,
  "username" text NOT NULL,
  "password" text NOT NULL,
  "create_at" timestamp without time zone NOT NULL,
  "last_login_at" timestamp without time zone NOT NULL,
  PRIMARY KEY ("id")
);

INSERT INTO "public"."users"("id", "username", "password", "create_at", "last_login_at") VALUES(1, 'admin', 'c0067d4af4e87f00dbac63b6156828237059172d1bbeac67427345d6a9fda484', '2018-10-10 14:01:35', '2018-10-10 14:01:35');
