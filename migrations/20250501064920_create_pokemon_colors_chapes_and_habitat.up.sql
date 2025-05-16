-- Your SQL goes here
CREATE TABLE pokemon_colors (
	id INTEGER NOT NULL, 
	identifier VARCHAR(50) NOT NULL, 
	PRIMARY KEY (id)
);

CREATE TABLE pokemon_habitats (
	id INTEGER NOT NULL, 
	identifier VARCHAR(50) NOT NULL, 
	PRIMARY KEY (id)
);

CREATE TABLE pokemon_shapes (
	id INTEGER NOT NULL, 
	identifier VARCHAR(50) NOT NULL, 
	PRIMARY KEY (id)
);

INSERT INTO "pokemon_colors" ("id", "identifier") VALUES
('1', 'black'),
('2', 'blue'),
('3', 'brown'),
('4', 'gray'),
('5', 'green'),
('6', 'pink'),
('7', 'purple'),
('8', 'red'),
('9', 'white'),
('10', 'yellow');

INSERT INTO "pokemon_habitats" ("id", "identifier") VALUES
('1', 'cave'),
('2', 'forest'),
('3', 'grassland'),
('4', 'mountain'),
('5', 'rare'),
('6', 'rough-terrain'),
('7', 'sea'),
('8', 'urban'),
('9', 'waters-edge');

INSERT INTO "pokemon_shapes" ("id", "identifier") VALUES
('1', 'ball'),
('2', 'squiggle'),
('3', 'fish'),
('4', 'arms'),
('5', 'blob'),
('6', 'upright'),
('7', 'legs'),
('8', 'quadruped'),
('9', 'wings'),
('10', 'tentacles'),
('11', 'heads'),
('12', 'humanoid'),
('13', 'bug-wings'),
('14', 'armor');