create table if not exists contenu (
             id integer primary key,
             titre text not null unique,
             url text,
             categorie text)


create table if not exists evenement (
             id integer primary key,
             titre text not null unique,
             niveau text)