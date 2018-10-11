-- Your SQL goes here

ALTER TABLE "public"."posts"
  ADD COLUMN "user_id" integer DEFAULT '1' NOT NULL,
  ADD COLUMN "publish_at" timestamp without time zone NOT NULL,
ADD COLUMN "url" text,
ADD CONSTRAINT "user_id" FOREIGN KEY ("user_id") REFERENCES "public"."users"("id");
