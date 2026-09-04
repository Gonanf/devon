# devon

> Proyecto de Gonanf — colección personal.
> **Lenguaje principal (GitHub):** Rust · **URL:** https://github.com/Gonanf/devon

## Qué es

Este repositorio forma parte de la colección de **Gonanf / Gabriel Solotorevsky** clonada en `/run/media/chaos/terciario/proyectos/devon`.

> **Nota:** README original preservado abajo en la sección "README original".

- **Path absoluto:** `/run/media/chaos/terciario/proyectos/devon`
- **Estado git:** último commit `2025-10-20 feat(Optimizing)`
- **Archivos (aprox):** 42
- **Stack detectado:** Rust (Cargo)

## Stack

- Rust (Cargo)

## Estructura

```
devon/
Cargo.lock
Cargo.toml
README.md
src/
  src/cli
  src/git
  src/main.rs
  src/p2p
```

## Cómo correr

> Instrucciones genéricas según el stack detectado. Ajustar según el repo.

```bash
cargo run
# o compilar
cargo build --release
```

## Estado

- **Último commit:** `2025-10-20 feat(Optimizing)`
- **Clonado en:** `/run/media/chaos/terciario/proyectos/devon`
- **Exclusiones del lote:** Forks, Workmatch, el-hornero-digital, mali/meli, Sherut (no tocados por consigna)

## Docs

- `docs/overview.md` — descripción extendida y guía rápida (generado en este lote)

## README original (preservado)

> Contenido previo de README.md recortado a 2000 chars para referencia:

```markdown

# Easier Readme

A simpler version of Sendme for learning




## Usage/Examples

```bash
A simpler version of Sendme for learning

Usage: easier_sendme <COMMAND>

Commands:
  send     
  receive  
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Example of the sending side:
```bash
easier_sendme send -p <PATH> -d <DATABASE>
```
Set <PATH> to the directory of file to send, and a <DATABASE> path to save and load the files/hashes.

If both are empty, it will open a temporal default.

Example of the receiver side:
```bash
easier_sendme receive <TICKET> -p <PATH> 
```
Get the <TICKET> from the sender and optionaly set a <PATH> to save those files.

You will not be able to set a database since a receiver only needs to get the data that is asking for.


## Run Locally

Clone the project

```bash
  git clone https://github.com/Gonanf/easier_sendme
```

Go to the project directory

```bash
  cd easier_sendme
```

Build the program

```bash
  cargo build
```



```

---
*README generado/mejorado automáticamente el 2026-09-04 con inspección de repo (opencode/agy pattern: lectura de estructura, lenguaje y entrypoints). No se modificó código, solo documentación.*
*Autor original: Gonanf — https://github.com/Gonanf/devon*
