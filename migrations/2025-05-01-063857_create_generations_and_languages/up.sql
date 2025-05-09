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