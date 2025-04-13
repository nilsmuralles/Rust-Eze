CREATE TABLE eventos (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(255) NOT NULL,
    fecha timestamp NOT NULL,
    updated_at timestamp NOT NULL,
    created_at timestamp NOT NULL
);

CREATE TABLE usuarios (
    id SERIAL PRIMARY KEY, 
    nombre VARCHAR(255) NOT NULL, 
    correo VARCHAR(255),
    updated_at timestamp NOT NULL,
    created_at timestamp NOT NULL
);

CREATE TABLE asientos (
    id SERIAL PRIMARY KEY,
    evento_id int NOT NULL REFERENCES eventos(id),
    updated_at timestamp NOT NULL,
    created_at timestamp NOT NULL
);

CREATE TABLE reservas (
    id SERIAL PRIMARY KEY,
    usuario_id int NOT NULL REFERENCES usuarios(id),
    evento_id int NOT NULL REFERENCES eventos(id),
    asiento_id int NOT NULL REFERENCES asientos(id), 
    updated_at timestamp NOT NULL,
    created_at timestamp NOT NULL
);