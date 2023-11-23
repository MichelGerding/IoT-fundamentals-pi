-- creaetion of databases
CREATE DATABASE mqtt_data;

CREATE TABLE mqtt_data.users (
	`id` INT unsigned NOT NULL AUTO_INCREMENT,
	`username` VARCHAR(255) NOT NULL,
	`password` VARCHAR(255) NOT NULL,
	PRIMARY KEY (`id`)
);

-- add a user into the databsae which the mqtt clients will connect to 
INSERT INTO mqtt_data.users (username, password) VALUES ('user1', '5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8');

CREATE DATABASE data;


create table data.measurements
(
    id        bigint unsigned auto_increment
        primary key,
    type      enum ('TEMPERATURE', 'HUMIDITY', 'PRESSURE') null,
    value     float                                        not null,
    send      tinyint(1) default 0                         not null,
    timestamp timestamp  default CURRENT_TIMESTAMP         not null,
    constraint id
        unique (id)
);

-- exampleto select the latest measurements of each type
-- with data as (
--     select
--         m.type,
--         m.value
--     from (
--              select type, MAX(timestamp) as tm
--              from measurements group by type
--          ) v join measurements m on v.type = m.type and v.tm = m.timestamp
-- ) select
--       ( select value from data where type = 'HUMIDITY') as HUMIDITY,
--       ( select value from data where type = 'PRESSURE') as PRESSURE,
--       ( select value from data where type = 'TEMPERATURE') as TEMPERATURE
