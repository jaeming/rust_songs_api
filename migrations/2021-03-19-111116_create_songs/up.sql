CREATE TABLE artists (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE songs (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  duration INT NOT NULL DEFAULT 0,
  artist_id INT NOT NULL,
  FOREIGN KEY (artist_id) REFERENCES artists(id)
);

INSERT INTO artists(id, name) VALUES (1, 'Mars');
INSERT INTO songs(title, duration, artist_id) VALUES ('Link', 140, 1);
INSERT INTO songs(title, duration, artist_id) VALUES ('Milky Way', 211, 1);
INSERT INTO songs(title, duration, artist_id) VALUES ('Malin', 308, 1);

INSERT INTO artists(id, name) VALUES (2, 'Pluto');
INSERT INTO songs(title, duration, artist_id) VALUES ('Ganondorf', 118, 2);
INSERT INTO songs(title, duration, artist_id) VALUES ('Bowser', 111, 2);
INSERT INTO songs(title, duration, artist_id) VALUES ('Mewtwo', 212, 2);