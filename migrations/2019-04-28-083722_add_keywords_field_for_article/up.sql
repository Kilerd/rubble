-- Your SQL goes here
ALTER TABLE "public"."articles" ADD COLUMN keywords TEXT[] NOT NULL DEFAULT '{}'::text[];