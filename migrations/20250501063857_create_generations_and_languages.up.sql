-- Your SQL goes here
CREATE TABLE generations (
	id INTEGER NOT NULL, 
	main_region_id INTEGER NOT NULL, 
	identifier VARCHAR(50) NOT NULL, 
	PRIMARY KEY (id)
);

CREATE TABLE languages (
	id INTEGER NOT NULL, 
	iso639 VARCHAR(10) NOT NULL, 
	iso3166 VARCHAR(10) NOT NULL, 
	identifier VARCHAR(50) NOT NULL, 
	official BOOLEAN NOT NULL, 
	"order" INTEGER NOT NULL, 
	PRIMARY KEY (id)
);

CREATE TABLE version_groups (
	id INTEGER NOT NULL, 
	identifier VARCHAR(50) NOT NULL, 
	generation_id INTEGER NOT NULL, 
	"order" INTEGER NOT NULL, 
	PRIMARY KEY (id), 
	FOREIGN KEY(generation_id) REFERENCES generations (id), 
	UNIQUE (identifier)
);

CREATE TABLE versions (
	id INTEGER NOT NULL, 
	version_group_id INTEGER NOT NULL, 
	identifier VARCHAR(50) NOT NULL, 
	PRIMARY KEY (id), 
	FOREIGN KEY(version_group_id) REFERENCES version_groups (id), 
	UNIQUE (identifier)
);

CREATE TABLE stats (
    id INTEGER NOT NULL,
    damage_class_id INTEGER,
    identifier VARCHAR(50) NOT NULL,
    is_battle_only BOOLEAN NOT NULL,
    game_index INTEGER,
    PRIMARY KEY (id)
);

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

INSERT INTO "generations" ("id", "main_region_id", "identifier") VALUES
('1', '1', 'generation-i'),
('2', '2', 'generation-ii'),
('3', '3', 'generation-iii'),
('4', '4', 'generation-iv'),
('5', '5', 'generation-v'),
('6', '6', 'generation-vi'),
('7', '7', 'generation-vii'),
('8', '8', 'generation-viii'),
('9', '10', 'generation-ix');

INSERT INTO "languages" ("id", "iso639", "iso3166", "identifier", "official", "order") VALUES
('1', 'ja', 'jp', 'ja-Hrkt', '1', '1'),
('2', 'ja', 'jp', 'roomaji', '1', '3'),
('3', 'ko', 'kr', 'ko', '1', '4'),
('4', 'zh', 'cn', 'zh-Hant', '1', '5'),
('5', 'fr', 'fr', 'fr', '1', '8'),
('6', 'de', 'de', 'de', '1', '9'),
('7', 'es', 'es', 'es', '1', '10'),
('8', 'it', 'it', 'it', '1', '11'),
('9', 'en', 'us', 'en', '1', '7'),
('10', 'cs', 'cz', 'cs', '1', '12'),
('11', 'ja', 'jp', 'ja', '1', '2'),
('12', 'zh', 'cn', 'zh-Hans', '1', '6'),
('13', 'pt-BR', 'br', 'pt-BR', '0', '13');

INSERT INTO "version_groups" ("id", "identifier", "generation_id", "order") VALUES
('1', 'red-blue', '1', '1'),
('2', 'yellow', '1', '2'),
('3', 'gold-silver', '2', '3'),
('4', 'crystal', '2', '4'),
('5', 'ruby-sapphire', '3', '5'),
('6', 'emerald', '3', '6'),
('7', 'firered-leafgreen', '3', '9'),
('8', 'diamond-pearl', '4', '10'),
('9', 'platinum', '4', '11'),
('10', 'heartgold-soulsilver', '4', '12'),
('11', 'black-white', '5', '13'),
('12', 'colosseum', '3', '7'),
('13', 'xd', '3', '8'),
('14', 'black-2-white-2', '5', '14'),
('15', 'x-y', '6', '15'),
('16', 'omega-ruby-alpha-sapphire', '6', '16'),
('17', 'sun-moon', '7', '17'),
('18', 'ultra-sun-ultra-moon', '7', '18'),
('19', 'lets-go-pikachu-lets-go-eevee', '7', '19'),
('20', 'sword-shield', '8', '20'),
('21', 'the-isle-of-armor', '8', '21'),
('22', 'the-crown-tundra', '8', '22'),
('23', 'brilliant-diamond-and-shining-pearl', '8', '23'),
('24', 'legends-arceus', '8', '24'),
('25', 'scarlet-violet', '9', '25'),
('26', 'the-teal-mask', '9', '26'),
('27', 'the-indigo-disk', '9', '27');

INSERT INTO "versions" ("id", "version_group_id", "identifier") VALUES
('1', '1', 'red'),
('2', '1', 'blue'),
('3', '2', 'yellow'),
('4', '3', 'gold'),
('5', '3', 'silver'),
('6', '4', 'crystal'),
('7', '5', 'ruby'),
('8', '5', 'sapphire'),
('9', '6', 'emerald'),
('10', '7', 'firered'),
('11', '7', 'leafgreen'),
('12', '8', 'diamond'),
('13', '8', 'pearl'),
('14', '9', 'platinum'),
('15', '10', 'heartgold'),
('16', '10', 'soulsilver'),
('17', '11', 'black'),
('18', '11', 'white'),
('19', '12', 'colosseum'),
('20', '13', 'xd'),
('21', '14', 'black-2'),
('22', '14', 'white-2'),
('23', '15', 'x'),
('24', '15', 'y'),
('25', '16', 'omega-ruby'),
('26', '16', 'alpha-sapphire'),
('27', '17', 'sun'),
('28', '17', 'moon'),
('29', '18', 'ultra-sun'),
('30', '18', 'ultra-moon'),
('31', '19', 'lets-go-pikachu'),
('32', '19', 'lets-go-eevee'),
('33', '20', 'sword'),
('34', '20', 'shield'),
('35', '21', 'the-isle-of-armor'),
('36', '22', 'the-crown-tundra'),
('37', '23', 'brilliant-diamond'),
('38', '23', 'shining-pearl'),
('39', '24', 'legends-arceus'),
('40', '25', 'scarlet'),
('41', '25', 'violet'),
('42', '26', 'the-teal-mask'),
('43', '27', 'the-indigo-disk');

INSERT INTO stats (id, damage_class_id, identifier, is_battle_only, game_index) VALUES
(1, NULL, 'hp', FALSE, 1),
(2, 2, 'attack', FALSE, 2),
(3, 2, 'defense', FALSE, 3),
(4, 3, 'special-attack', FALSE, 5),
(5, 3, 'special-defense', FALSE, 6),
(6, NULL, 'speed', FALSE, 4),
(7, NULL, 'accuracy', TRUE, NULL),
(8, NULL, 'evasion', TRUE, NULL);


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