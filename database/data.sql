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

-- Asientos (30 por evento)
-- Evento 1: asientos 1 al 30
INSERT INTO asientos (id, evento_id, updated_at, created_at)
VALUES
-- Evento 1
(1, 1, now(), now()), (2, 1, now(), now()), (3, 1, now(), now()), (4, 1, now(), now()), (5, 1, now(), now()),
(6, 1, now(), now()), (7, 1, now(), now()), (8, 1, now(), now()), (9, 1, now(), now()), (10, 1, now(), now()),
(11, 1, now(), now()), (12, 1, now(), now()), (13, 1, now(), now()), (14, 1, now(), now()), (15, 1, now(), now()),
(16, 1, now(), now()), (17, 1, now(), now()), (18, 1, now(), now()), (19, 1, now(), now()), (20, 1, now(), now()),
(21, 1, now(), now()), (22, 1, now(), now()), (23, 1, now(), now()), (24, 1, now(), now()), (25, 1, now(), now()),
(26, 1, now(), now()), (27, 1, now(), now()), (28, 1, now(), now()), (29, 1, now(), now()), (30, 1, now(), now());

-- Evento 2: asientos 31 al 60
INSERT INTO asientos (id, evento_id, updated_at, created_at)
VALUES
(31, 2, now(), now()), (32, 2, now(), now()), (33, 2, now(), now()), (34, 2, now(), now()), (35, 2, now(), now()),
(36, 2, now(), now()), (37, 2, now(), now()), (38, 2, now(), now()), (39, 2, now(), now()), (40, 2, now(), now()),
(41, 2, now(), now()), (42, 2, now(), now()), (43, 2, now(), now()), (44, 2, now(), now()), (45, 2, now(), now()),
(46, 2, now(), now()), (47, 2, now(), now()), (48, 2, now(), now()), (49, 2, now(), now()), (50, 2, now(), now()),
(51, 2, now(), now()), (52, 2, now(), now()), (53, 2, now(), now()), (54, 2, now(), now()), (55, 2, now(), now()),
(56, 2, now(), now()), (57, 2, now(), now()), (58, 2, now(), now()), (59, 2, now(), now()), (60, 2, now(), now());

-- Evento 3: asientos 61 al 90
INSERT INTO asientos (id, evento_id, updated_at, created_at)
VALUES
(61, 3, now(), now()), (62, 3, now(), now()), (63, 3, now(), now()), (64, 3, now(), now()), (65, 3, now(), now()),
(66, 3, now(), now()), (67, 3, now(), now()), (68, 3, now(), now()), (69, 3, now(), now()), (70, 3, now(), now()),
(71, 3, now(), now()), (72, 3, now(), now()), (73, 3, now(), now()), (74, 3, now(), now()), (75, 3, now(), now()),
(76, 3, now(), now()), (77, 3, now(), now()), (78, 3, now(), now()), (79, 3, now(), now()), (80, 3, now(), now()),
(81, 3, now(), now()), (82, 3, now(), now()), (83, 3, now(), now()), (84, 3, now(), now()), (85, 3, now(), now()),
(86, 3, now(), now()), (87, 3, now(), now()), (88, 3, now(), now()), (89, 3, now(), now()), (90, 3, now(), now());

