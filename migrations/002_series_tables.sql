CREATE TABLE IF NOT EXISTS "series" (
  "id" varchar(255),
  "name" varchar(255),
  "count" bigint
);

CREATE TABLE IF NOT EXISTS "book_series" (
    "book_id" bigint,
    "series_id" bigint,
    "name" varchar(255),
    "series_number" integer
);
