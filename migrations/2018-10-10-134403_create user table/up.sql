-- Your SQL goes here
CREATE TABLE users (
  "id" serial,
  "username" text,
  "password" text,
  "create_at" timestamp without time zone,
  "last_login_at" timestamp without time zone,
  PRIMARY KEY ("id")
);

INSERT INTO "public"."users"("id", "username", "password", "create_at", "last_login_at") VALUES(1, 'admin', 'admin', '2018-10-10 14:01:35', '2018-10-10 14:01:35');
