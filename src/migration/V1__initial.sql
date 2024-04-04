create table reference (
             id VARCHAR(50) primary key,
             nom text not null unique,
             url text,
             categorie text);

create table if not exists tag (
             id VARCHAR(50) primary key,
             nom text not null,
             reference_id VARCHAR(50),
             FOREIGN KEY(reference_id) REFERENCES reference(id));