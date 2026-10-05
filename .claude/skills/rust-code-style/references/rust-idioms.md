# Rust-Specific Guidelines

## Contents
- [Prefer explicit types in signatures](#prefer-explicit-types-in-signatures)
- [Use type aliases for complex types](#use-type-aliases-for-complex-types)
- [Leverage the newtype pattern](#leverage-the-newtype-pattern)
- [Use enums for state](#use-enums-for-state)
- [Avoid unwrap() and expect() in production code](#avoid-unwrap-and-expect-in-production-code)
- [Use descriptive error context](#use-descriptive-error-context)
- [Pattern matching should be exhaustive](#pattern-matching-should-be-exhaustive)
- [Use #[must_use] for important results](#use-must_use-for-important-results)
- [Prefer iterators over index loops](#prefer-iterators-over-index-loops)
- [Use ? instead of manual match](#use--instead-of-manual-match)
- [Use let-else for early returns with destructuring](#use-let-else-for-early-returns-with-destructuring)
- [Use LazyLock and OnceLock for lazy statics](#use-lazylock-and-oncelock-for-lazy-statics)
- [Use NonZero<T> to make invalid numeric states unrepresentable](#use-nonzerot-to-make-invalid-numeric-states-unrepresentable)
- [Use #[non_exhaustive] for public enums in libraries](#use-non_exhaustive-for-public-enums-in-libraries)

## Prefer explicit types in signatures

Prefer `async fn` over returning `impl Future` or `BoxFuture` — it's clearer and avoids unnecessary boxing. Reserve `BoxFuture` for trait objects or when you must erase the future type across an API boundary.

When using `impl Trait` in return position with captured lifetimes (Rust 1.82+), use the `use<>` precise-capturing syntax to be explicit about what lifetimes the opaque type captures.

**DON'T:**
```rust
pub fn process(data: impl Serialize) -> BoxFuture<'static, Result<Response>>
```

**DO:**
```rust
pub async fn process(data: RequestData) -> Result<Response>

// When returning impl Trait with lifetimes (1.82+)
fn active_users<'a>(&'a self) -> impl Iterator<Item = &User> + use<'a> { ... }
```

## Use type aliases for complex types

**DON'T:**
```rust
fn handler() -> Pin<Box<dyn Future<Output = Result<Response, Error>> + Send>>
```

**DO:**
```rust
type HandlerFuture = Pin<Box<dyn Future<Output = Result<Response, Error>> + Send>>;

fn handler() -> HandlerFuture
```

## Leverage the newtype pattern

Make invalid states unrepresentable.

**DON'T:**
```rust
fn send_email(email: String) -> Result<()>
// Any string can be passed, even invalid emails
```

**DO:**
```rust
struct ValidatedEmail(String);

impl ValidatedEmail {
    fn new(email: String) -> Result<Self, ValidationError> {
        // validate email format
        Ok(Self(email))
    }
}

fn send_email(email: ValidatedEmail) -> Result<()>
// Only validated emails can be passed
```

## Use enums for state

**DON'T:**
```rust
struct User {
    status: String,  // "active", "suspended", "deleted"?
    deleted_at: Option<DateTime>,
    suspension_reason: Option<String>,
}
```

**DO:**
```rust
enum UserStatus {
    Active,
    Suspended { reason: String, until: DateTime },
    Deleted { deleted_at: DateTime },
}

struct User {
    status: UserStatus,
}
```

## Avoid unwrap() and expect() in production code

**DON'T:**
```rust
let user = db.get_user(id).unwrap();
let config = load_config().expect("config must exist");
```

**DO:**
```rust
let user = db.get_user(id)?;
let config = load_config()
    .map_err(|e| Error::Config(format!("Failed to load config: {e}")))?;
```

## Use descriptive error context

**DON'T:**
```rust
db.fetch_user(id).map_err(|e| Error::Database(e))?
```

**DO:**
```rust
db.fetch_user(id)
    .map_err(|e| Error::Database(format!("Failed to fetch user {id}: {e}")))?
```

## Pattern matching should be exhaustive

**DON'T:**
```rust
match status {
    Status::Active => process(),
    _ => {}  // Silent failure for all other cases
}
```

**DO:**
```rust
match status {
    Status::Active => process(),
    Status::Suspended => handle_suspended(),
    Status::Deleted => Err(Error::UserDeleted),
}
```

## Use #[must_use] for important results

```rust
#[must_use = "transaction must be committed or rolled back"]
pub struct Transaction { ... }

#[must_use = "iterator is lazy and does nothing unless consumed"]
pub fn process_items(&self) -> impl Iterator<Item = ProcessedItem>
```

## Prefer iterators over index loops

**DON'T:**
```rust
for i in 0..items.len() {
    process(&items[i]);
}
```

**DO:**
```rust
for item in &items {
    process(item);
}
```

## Use ? instead of manual match

**DON'T:**
```rust
let user = match db.get_user(id) {
    Ok(u) => u,
    Err(e) => return Err(e.into()),
};
```

**DO:**
```rust
let user = db.get_user(id)?;
```

Note: `?` relies on `From` for error conversion. Implement `From<SourceError> for YourError` to enable propagation across error types without manual `.map_err()`.

## Use let-else for early returns with destructuring

`let-else` (stable since 1.65) is the idiomatic guard clause when you need to destructure and return early if the pattern doesn't match. It keeps the happy path at the top level without nesting.

**DON'T:**
```rust
fn process(event: Option<Event>) -> Result<()> {
    if let Some(event) = event {
        // happy path deeply nested
        handle(event)?;
    }
    Ok(())
}
```

**DO:**
```rust
fn process(event: Option<Event>) -> Result<()> {
    let Some(event) = event else {
        return Ok(());
    };
    handle(event)
}
```

Also useful for enum variants:
```rust
let Status::Active { since } = user.status else {
    return Err(Error::UserNotActive);
};
```

## Use LazyLock and OnceLock for lazy statics

Since Rust 1.80, `std::sync::LazyLock` and `std::sync::OnceLock` are stable in std — prefer them over the `once_cell` or `lazy_static` crates.

```rust
// LazyLock: value computed on first access
static CONFIG: std::sync::LazyLock<Config> =
    std::sync::LazyLock::new(|| Config::load().expect("config must be valid at startup"));

// OnceLock: value set exactly once at runtime
static FLAG: std::sync::OnceLock<bool> = std::sync::OnceLock::new();

fn is_enabled() -> bool {
    *FLAG.get_or_init(|| std::env::var("FEATURE").is_ok())
}
```

## Use NonZero\<T\> to make invalid numeric states unrepresentable

`std::num::NonZero<T>` (unified type since 1.79, previously `NonZeroU32` etc.) encodes the constraint in the type system, eliminating runtime checks.

```rust
use std::num::NonZero;

// DON'T
fn set_page_size(size: u32) {
    assert!(size > 0, "page size must be non-zero");
    // ...
}

// DO
fn set_page_size(size: NonZero<u32>) {
    // guaranteed non-zero by the type
}
```

## Use #[non_exhaustive] for public enums in libraries

Mark public enums `#[non_exhaustive]` when you may add variants in future versions. This forces downstream callers to include a wildcard arm, preventing breakage when you extend the enum.

```rust
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    NotFound,
    PermissionDenied,
    // future variants won't break downstream match expressions
}
```

Do **not** use `#[non_exhaustive]` on internal enums — exhaustive matching is a feature there, catching unhandled cases at compile time.
