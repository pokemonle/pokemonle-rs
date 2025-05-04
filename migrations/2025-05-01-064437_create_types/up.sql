-- Your SQL goes here
CREATE TABLE move_damage_classes (
	id INTEGER NOT NULL, 
	identifier VARCHAR(50) NOT NULL, 
	PRIMARY KEY (id)
);

CREATE TABLE types (
	id INTEGER NOT NULL, 
	identifier VARCHAR(50) NOT NULL, 
	generation_id INTEGER NOT NULL, 
	damage_class_id INTEGER, 
	PRIMARY KEY (id), 
	FOREIGN KEY(damage_class_id) REFERENCES move_damage_classes (id), 
	FOREIGN KEY(generation_id) REFERENCES generations (id), 
	UNIQUE (identifier)
);

INSERT INTO "move_damage_classes" ("id", "identifier") VALUES
('1', 'status'),
('2', 'physical'),
('3', 'special');

INSERT INTO "types" ("id", "identifier", "generation_id", "damage_class_id") VALUES
('1', 'normal', '1', '2'),
('2', 'fighting', '1', '2'),
('3', 'flying', '1', '2'),
('4', 'poison', '1', '2'),
('5', 'ground', '1', '2'),
('6', 'rock', '1', '2'),
('7', 'bug', '1', '2'),
('8', 'ghost', '1', '2'),
('9', 'steel', '2', '2'),
('10', 'fire', '1', '3'),
('11', 'water', '1', '3'),
('12', 'grass', '1', '3'),
('13', 'electric', '1', '3'),
('14', 'psychic', '1', '3'),
('15', 'ice', '1', '3'),
('16', 'dragon', '1', '3'),
('17', 'dark', '2', '3'),
('18', 'fairy', '6', NULL),
('19', 'stellar', '9', NULL),
('10001', 'unknown', '2', NULL),
('10002', 'shadow', '3', NULL);

