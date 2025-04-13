INSERT INTO eventos (id, nombre, fecha, updated_at, created_at)
VALUES
(1, 'Concierto de Primavera', '2025-05-15 19:00:00', now(), now()),
(2, 'Festival de Tecnología', '2025-06-20 10:00:00', now(), now()),
(3, 'Feria del Libro', '2025-07-10 09:00:00', now(), now());


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


INSERT INTO asientos (id, evento_id, updated_at, created_at)
VALUES
(1, 1, now(), now()), (2, 1, now(), now()), (3, 1, now(), now()), (4, 1, now(), now()), (5, 1, now(), now()),
(6, 1, now(), now()), (7, 1, now(), now()), (8, 1, now(), now()), (9, 1, now(), now()), (10, 1, now(), now());


INSERT INTO asientos (id, evento_id, updated_at, created_at)
VALUES
(11, 2, now(), now()), (12, 2, now(), now()), (13, 2, now(), now()), (14, 2, now(), now()), (15, 2, now(), now()),
(16, 2, now(), now()), (17, 2, now(), now()), (18, 2, now(), now()), (19, 2, now(), now()), (20, 2, now(), now());


INSERT INTO asientos (id, evento_id, updated_at, created_at)
VALUES
(21, 3, now(), now()), (22, 3, now(), now()), (23, 3, now(), now()), (24, 3, now(), now()), (25, 3, now(), now()),
(26, 3, now(), now()), (27, 3, now(), now()), (28, 3, now(), now()), (29, 3, now(), now()), (30, 3, now(), now());


INSERT INTO reservas (id, usuario_id, evento_id, asiento_id, updated_at, created_at)
VALUES
(1, 1, 1, 1, now(), now()),
(2, 2, 1, 2, now(), now()),
(3, 3, 1, 3, now(), now()),
(4, 4, 1, 4, now(), now()),
(5, 5, 1, 5, now(), now()),
(6, 6, 1, 6, now(), now()),
(7, 7, 1, 7, now(), now());

INSERT INTO reservas (id, usuario_id, evento_id, asiento_id, updated_at, created_at)
VALUES
(8, 8, 2, 11, now(), now()),
(9, 9, 2, 12, now(), now()),
(10, 10, 2, 13, now(), now()),
(11, 1, 2, 14, now(), now());

INSERT INTO reservas (id, usuario_id, evento_id, asiento_id, updated_at, created_at)
VALUES
(12, 2, 3, 21, now(), now()),
(13, 3, 3, 22, now(), now()),
(14, 4, 3, 23, now(), now()),
(15, 5, 3, 24, now(), now()),
(16, 6, 3, 25, now(), now());
