# Rust-Eze

# Simulación en Rust

Este proyecto contiene una simulación desarrollada en Rust que utiliza Docker para levantar la base de datos en PostgreSQL. A continuación se detallan los pasos para ejecutar correctamente el entorno y correr la simulación.

---

## Requisitos

- Tener un WSL o estar en un ambiente Linux
- Tener Docker instalado y funcionando dentro de WSL.
- Tener Rust y Cargo instalados.

---

## Verifica si tienes Rust y Cargo

Ejecuta en tu terminal WSL:

```bash
rustc --version
cargo --version
```

## Si no tienes, puedes usar (en wsl):

```bash
sudo apt install rustup
sudo apt install cargo
rustup default stable
```

## para correr el programa

```bash
docker compose up --build -d

cargo build --release

cargo run
```

## Cuando hayas terminado apaga el docker ;)

```bash
docker compose down
```