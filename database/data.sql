INSERT INTO eventos (id, nombre, fecha, updated_at, created_at)
VALUES
(1, 'Concierto de Primavera', '2025-05-15 19:00:00', now(), now());

INSERT INTO usuarios (id, nombre, correo, updated_at, created_at)
VALUES
(1, 'Ana Pérez', 'ana.perez@example.com', now(), now()),
(2, 'Luis Gómez', 'luis.gomez@example.com', now(), now()),
(3, 'María López', 'maria.lopez@example.com', now(), now()),
(4, 'Carlos Méndez', 'carlos.mendez@example.com', now(), now()),
(5, 'Sofía Herrera', 'sofia.herrera@example.com', now(), now()),
(6, 'Daniel Cruz', 'daniel.cruz@example.com', now(), now()),
(7, 'Laura Jiménez', 'laura.jimenez@example.com', now(), now()),
(8, 'Ricardo Morales', 'ricardo.morales@example.com', now(), now()),
(9, 'Fernanda Díaz', 'fernanda.diaz@example.com', now(), now()),
(10, 'Jorge Ramírez', 'jorge.ramirez@example.com', now(), now());

-- Asientos (30 por evento)
INSERT INTO asientos (id, evento_id, updated_at, created_at)
VALUES
-- Evento 1
(1, 1, now(), now()), (2, 1, now(), now()), (3, 1, now(), now()), (4, 1, now(), now()), (5, 1, now(), now()),
(6, 1, now(), now()), (7, 1, now(), now()), (8, 1, now(), now()), (9, 1, now(), now()), (10, 1, now(), now()),
(11, 1, now(), now()), (12, 1, now(), now()), (13, 1, now(), now()), (14, 1, now(), now()), (15, 1, now(), now()),
(16, 1, now(), now()), (17, 1, now(), now()), (18, 1, now(), now()), (19, 1, now(), now()), (20, 1, now(), now()),
(21, 1, now(), now()), (22, 1, now(), now()), (23, 1, now(), now()), (24, 1, now(), now()), (25, 1, now(), now()),
(26, 1, now(), now()), (27, 1, now(), now()), (28, 1, now(), now()), (29, 1, now(), now()), (30, 1, now(), now());
