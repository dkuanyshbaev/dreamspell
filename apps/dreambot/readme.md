### dreambot

CREATE TABLE users (
    id integer PRIMARY KEY,
    birthday text Not null
);
CREATE TABLE seals (
    id integer PRIMARY KEY AUTOINCREMENT,
    name text Not null,
    image text Not null,
    archetype text Not null,
    archetype_description text Not null,
    portrait_description text Not null,
    type_description text Not null
);
