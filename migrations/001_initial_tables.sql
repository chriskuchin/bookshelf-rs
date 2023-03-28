CREATE TABLE IF NOT EXISTS "books" (
    "id" integer primary key autoincrement,
    "created_at" datetime,
    "updated_at" datetime,
    "deleted_at" datetime,
    "uuid" varchar(255) NOT NULL,
    "isbn" varchar(255),
    "title" varchar(255),
    "author" varchar(255),
    "description" varchar(255),
    "cover_url" varchar(255),
    "publisher" varchar(255),
    "pub_date" varchar(255)
);

CREATE TABLE IF NOT EXISTS "files" (
    "id" integer primary key autoincrement,
    "created_at" datetime,
    "updated_at" datetime,
    "deleted_at" datetime,
    "book_id" bigint,
    "mime_type" varchar(255),
    "path" varchar(255)
);
