# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project context

`func` is an experimental Rust library exploring functional programming abstractions (HKT emulation, Functor/Applicative/Monad, HList, Free/Trampoline, IO/Effect, Validation, TailRec). It is a personal playground — code is intentionally exploratory, includes commented-out experiments (notably the `Async` attempts in `src/effect.rs` and `ETFree` in `src/free.rs`), and is not intended for production use. README explicitly inspired by Rustz, Kinder, Frunk, tailrec.rs, and hlist.

The crate is on Rust **edition 2024**. Intra-crate module paths must be `crate::`-prefixed (e.g. `use crate::hkt::*;`, never `use hkt::*;`). Trait objects must use the `dyn` keyword (`Box<dyn Fn() -> A>`, `Rc<dyn Fn(A) -> Free<A>>`). The crate-wide `assert_eq!` shadowing from `pretty_assertions` is wired up via `#[macro_use] extern crate pretty_assertions;` in `lib.rs` — that older form still works in edition 2024 and is preserved intentionally so individual test modules don't need to re-import it.

## Common commands

- Build: `cargo build`
- Run all tests: `cargo test`
- Run tests in one module: `cargo test --lib <module>` (e.g. `cargo test --lib free`)
- Run a single test: `cargo test <test_fn_name>` (e.g. `cargo test test_free`)
- Show test output: `cargo test -- --nocapture`

There is no separate lint/format config — use `cargo clippy` and `cargo fmt` if needed. The only dev-dependency is `pretty_assertions ^1`. A clean build emits ~16 `dead_code` warnings on private items (e.g. `SimpleIO`, `HList`/`HNil`/`HCons`, `Show`, `IOError`) that are referenced only from `#[test]` blocks; these are pre-existing and intentional — do not "fix" them by deleting the code.

## Architecture

The crate is a flat collection of modules in `src/` re-exported from `src/lib.rs`. The conceptual layering matters more than the file layout:

1. **`hkt.rs` — Higher-Kinded Type emulation.** `HigherKindedType<V>` carries three associated types: `Current` (the current type parameter `T` in `F<T>`), `Output` (the target `V`), and `FOutput` (`F<V>`). The `hkt!`, `hkt_partial_left!`, `hkt_partial_right!` macros generate impls for one-param, left-fixed, and right-fixed two-param constructors. Every typeclass below builds on this trait; do **not** add a typeclass without first wiring up the corresponding HKT impl.

2. **Typeclass hierarchy** (each is a supertrait of the next):
   `Functor` (`functor.rs`) → `Apply` / `Applicative` (`applicative.rs`) → `Monad` (`monad.rs`).
   Implementations are provided for `Option`, `Result`, `Vec`, `Box`. The `functorize!` macro derives `Functor` for any iterable collection. Note `Apply::ap` has a deliberately ugly signature using fully-qualified `<Self as HigherKindedType<Fun>>::FOutput` — preserve this shape when adding new instances.

3. **Algebraic structures.** `semigroup.rs` defines `Semigroup` with `add_and_own` (consuming combine); `semigroup_num!` covers all numeric primitives, `semigroup!` covers `Extend`-able collections. `monoid.rs` builds on it. `foldable.rs` provides `Foldable` using these.

4. **Effects / stack-safety stack.** This is the most subtle area:
   - `lazy.rs` — `Lazy<A>` wraps `Box<dyn Fn() -> A>` with `map`/`flat_map`. The `lazy!` macro is the canonical constructor.
   - `trampoline.rs` and `tailrec.rs` — both implement looped recursion. `TailRec` is a blanket-impl trait giving every `T` a `.rec(...)` method; `tail_rec(input, iterate)` is the free-function form. Use these instead of recursion for stack safety.
   - `free.rs` — `Free<A>` enum (`Return | Suspend | FlatMap`) interpreted via `tail_rec`. Same-type-only (`Free<A>` not `Free<F,A>`) — the commented `ETFree` block shows an abandoned attempt at type-changing `FlatMap` via `transmute`. The `FlatMap` arm uses `Rc<dyn Fn>` (not `Box<dyn Fn>`) because the associativity rewrite in `run` needs to clone the continuation into a new closure.
   - `effect.rs` — Two parallel IO designs coexist: a simple eager `SimpleIO` (private), and a lazy `IO` trait with `Unit`/`Suspend`/`Map`/`FlatMap` ADT-style structs. The large commented-out `Async` blocks document a stuck attempt at an asynchronous IO referenced from http://degoes.net/articles/only-one-io — leave them as-is unless explicitly asked to revisit.
   - `io.rs` — minimal early sketch.

5. **HList** (`hlist.rs`) — two parallel encodings: the type-level `HCons<T, V: HList>` / `HNil` (with `hlist![...]` value macro and `Hlist![...]` type macro), and a simpler runtime `EHList<T,V>` enum. Both are kept; pick the type-level one for compile-time-known shapes.

6. **Validation** (`validation.rs`) — `Validation<T,E>` accumulates errors as `Vec<E>` via `append`, unlike `Result` which short-circuits. Has its own `map`/`and_then`/`map_err`; not yet integrated with the typeclass hierarchy.

7. **Helpers**: `option_ops.rs`, `result_ops.rs` add ergonomic `to_some`/`to_ok`/`to_err` constructors used in tests; `show.rs` defines `Show`/`ShowDebug`.

## Conventions to match when editing

- Tests live inline next to the code they test (`#[test] fn test_*` at the bottom of each file), not in a separate `tests/` dir.
- New typeclass instances must implement the impl for `Option`, `Result`, `Vec`, and `Box` to match the existing pattern.
- Macros are exported with `#[macro_export]` and used across modules — they rely on `$crate::module::Name` paths.
- Many experiments are preserved as commented blocks with links/notes; don't delete these without checking with the user.
