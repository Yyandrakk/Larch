# ADR-0003: Database Access Strategy (ORM vs Raw SQL)

## Context

We are building a desktop application with local SQLite storage. The user has a strong background in .NET/Entity Framework (EF) and prefers a declarative, type-safe approach with migrations over writing raw SQL strings.
Currently, we are using `rusqlite` which involves writing raw SQL queries.

## Options

### 1. Rusqlite (Current)

- **Pros:** Lightweight, simple, standard for SQLite in Rust. No async overhead (though we run it in async commands).
- **Cons:** Raw SQL strings, manual mapping, no built-in migrations (need separate tool like `refinery`), less type safety.
- **Verdict:** Good for simple apps, but lacks the "Enterprise" feel of EF.

### 2. Diesel

- **Pros:** Mature, strong compile-time checking, synchronous (good for CPU-bound, but we are in async context).
- **Cons:** Synchronous API (blocks async runtime if not careful), complex type system, steep learning curve.
- **Verdict:** Powerful but maybe too rigid and sync-only.

### 3. SeaORM

- **Pros:** Async (built on SQLx), dynamic (like EF), supports migrations, entity-based, intuitive for EF users.
- **Cons:** Slightly heavier runtime (dynamic dispatch), newer than Diesel.
- **Verdict:** The closest experience to EF Core in the Rust ecosystem.

## Decision

We will continue with **Rusqlite** for the immediate "Project Configuration" milestone to avoid stalling progress, but we acknowledge that **SeaORM** is the better long-term fit for the project's architectural goals and user persona.

We will plan a **migration to SeaORM** in a subsequent milestone (M1.5 or M2) to introduce:

1.  Type-safe Entity definitions (Structs).
2.  Declarative migrations.
3.  Async-native database access.

## Status

Accepted (Transition Plan)
