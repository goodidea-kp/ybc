# Function Design

## Contents
- [Keep functions short](#keep-functions-short)
- [Single Responsibility Principle](#single-responsibility-principle)
- [Limit function parameters](#limit-function-parameters)
- [No deep nesting](#no-deep-nesting)

## Keep functions short

**Target:** 5–15 lines. **Maximum:** 25 lines. If longer, extract smaller functions with descriptive names.

## Single Responsibility Principle

Each function does one thing well.

**DON'T:**
```rust
fn process_user(id: i32) -> Result<()> {
    let user = db.fetch(id)?;
    if user.email.is_empty() { return Err(Error::Invalid); }
    let perms = calc_perms(&user)?;
    db.update_perms(user.id, perms)?;
    email::send(&user.email, "Welcome")?;
    log::info("User processed: {}", user.id);
    cache.invalidate(user.id);
    Ok(())
}
```

**DO:**
```rust
fn process_user(id: i32) -> Result<()> {
    let user = fetch_and_validate_user(id)?;
    grant_user_permissions(&user)?;
    notify_user_by_email(&user)?;
    log_user_activity(&user)?;
    invalidate_user_cache(&user)?;
    Ok(())
}
```

## Limit function parameters

**Maximum:** 3 parameters. If more are needed, use a struct/config object.

**DON'T:**
```rust
fn create_user(
    name: String,
    email: String,
    age: i32,
    role: String,
    dept: String,
    manager: String,
    location: String
) -> User
```

**DO:**
```rust
struct CreateUserRequest {
    name: String,
    email: String,
    age: i32,
    role: String,
    department: String,
    manager: String,
    location: String,
}

fn create_user(request: CreateUserRequest) -> User
```

## No deep nesting

**Maximum nesting level:** 2. Use early returns, guard clauses, and `let-else` for destructuring guards (stable since 1.65).

**DON'T:**
```rust
fn process(data: Option<Data>) -> Result<()> {
    if let Some(d) = data {
        if d.is_valid() {
            if d.has_permission() {
                if d.is_ready() {
                    // deeply nested work
                }
            }
        }
    }
    Ok(())
}
```

**DO:**
```rust
fn process(data: Option<Data>) -> Result<()> {
    let data = data.ok_or(Error::NoData)?;

    if !data.is_valid() {
        return Err(Error::Invalid);
    }
    if !data.has_permission() {
        return Err(Error::Forbidden);
    }
    if !data.is_ready() {
        return Err(Error::NotReady);
    }

    // work at top level
    Ok(())
}
```

When the guard involves destructuring, prefer `let-else` over `if let` + nesting:

```rust
fn process(event: Option<Event>) -> Result<()> {
    let Some(event) = event else {
        return Ok(());
    };
    // event is in scope here, no extra nesting
    handle(event)
}
```
