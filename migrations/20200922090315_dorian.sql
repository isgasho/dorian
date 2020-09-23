CREATE TABLE users (
	id       SERIAL PRIMARY KEY,
	name     VARCHAR NOT NULL
);

CREATE TABLE entries (
	id       SERIAL PRIMARY KEY,
	uploader VARCHAR NOT NULL
);

CREATE TABLE tags (
	id       SERIAL PRIMARY KEY,
	name     VARCHAR NOT NULL UNIQUE
);

CREATE TABLE tagmap (
	id       SERIAL PRIMARY KEY,
	tag_id   INTEGER NOT NULL,
	entry_id INTEGER NOT NULL,

	FOREIGN KEY(tag_id) REFERENCES tags(id),
	FOREIGN KEY(entry_id) REFERENCES entries(id),
	UNIQUE(tag_id,entry_id) -- to make sure there are no duplicate rows
);
