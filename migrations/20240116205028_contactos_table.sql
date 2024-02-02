-- Add migration script here
CREATE TABLE contactos (
    id SERIAL PRIMARY KEY,
    nome TEXT NOT NULL,
    email TEXT NOT NULL,
    assunto TEXT NOT NULL,
    texto TEXT NOT NULL,
    tempo TIMESTAMPTZ NOT NULL
);