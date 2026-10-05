# Code Organization, Comments, Errors, Testing & Anti-Patterns

## Contents
- [Module structure](#module-structure)
- [File size limits](#file-size-limits)
- [Group related code together](#group-related-code-together)
- [Comments: why, not what](#comments-why-not-what)
- [Document all public APIs](#document-all-public-apis)
- [Remove dead code](#remove-dead-code)
- [Make errors explicit and specific](#make-errors-explicit-and-specific)
- [Provide context in error messages](#provide-context-in-error-messages)
- [Testing](#testing)
- [Cognitive load reduction](#cognitive-load-reduction)
- [Anti-patterns to avoid](#anti-patterns-to-avoid)

## Module structure

```rust
// Consistent ordering:
// 1. Module-level documentation
// 2. Imports (grouped: std, external crates, internal crates, current crate)
// 3. Constants
// 4. Type definitions (structs, enums)
// 5. Trait implementations
// 6. Public functions
// 7. Private functions
// 8. Tests module

use std::collections::HashMap;

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::database::Database;
use crate::error::Error;

const MAX_RETRIES: u32 = 3;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

// ... rest of module
```

## File size limits

**Target:** 200–300 lines. **Maximum:** 500 lines. Split larger files by responsibility into separate modules.

## Group related code together

Keep related functions, types, and constants close to each other.

## Comments: why, not what

**DON'T:**
```rust
// Increment counter
counter += 1;

// Loop through users
for user in users { ... }
```

**DO:**
```rust
// Account for zero-indexed array in user-facing display
counter += 1;

// Process only active users to avoid triggering suspended account emails
for user in users.iter().filter(|u| u.is_active()) { ... }
```

## Document all public APIs

```rust
/// Fetches a user by their unique identifier.
///
/// # Arguments
/// * `user_id` - The unique user identifier
///
/// # Returns
/// * `Ok(User)` - The user if found
/// * `Err(Error::NotFound)` - If no user exists with that ID
/// * `Err(Error::Database)` - If database connection fails
///
/// # Example
/// ```
/// let user = get_user(42)?;
/// println!("Found user: {}", user.name);  // Display trait — human-readable
/// ```
pub async fn get_user(user_id: i32) -> Result<User, Error> {
    // implementation
}
```

## Remove dead code

Don't comment out code. Delete it. Version control remembers.

## Make errors explicit and specific

**DON'T:**
```rust
fn get_user(id: i32) -> Option<User>  // Why None? Not found? DB error?

enum Error {
    Failed,  // What failed?
}
```

**DO:**
```rust
fn get_user(user_id: i32) -> Result<User, Error>

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("User {0} not found")]
    UserNotFound(i32),

    #[error("Database connection failed: {0}")]
    DatabaseConnection(String),

    #[error("Invalid user data: {0}")]
    InvalidData(String),
}
```

## Provide context in error messages

```rust
database
    .fetch_user(user_id)
    .await
    .map_err(|error| Error::Database(
        format!("Failed to fetch user {user_id} from database: {error}")
    ))?
```

## Testing

### Test names should be descriptive sentences

**DON'T:**
```rust
#[test]
fn test1() { ... }

#[test]
fn test_user() { ... }

#[test]
fn auth() { ... }
```

**DO:**
```rust
#[test]
fn authenticated_user_can_access_protected_endpoint() { ... }

#[test]
fn unauthenticated_request_returns_401() { ... }

#[test]
fn deleted_user_cannot_login() { ... }
```

### One logical assert per test

Focus each test on a single behavior.

**DON'T:**
```rust
#[test]
fn test_user() {
    let user = create_user();
    assert!(user.is_valid());
    assert_eq!(user.email, "test@example.com");
    assert!(user.can_login());
    assert_eq!(user.role, Role::User);
}
```

**DO:**
```rust
#[test]
fn newly_created_user_is_valid() {
    let user = create_user();
    assert!(user.is_valid());
}

#[test]
fn newly_created_user_has_correct_email() {
    let user = create_user();
    assert_eq!(user.email, "test@example.com");
}
```

### Use descriptive test data

**DON'T:**
```rust
let user = User { name: "a", age: 1, email: "b" };
```

**DO:**
```rust
let admin_user = User {
    name: "Admin Smith",
    age: 35,
    email: "admin@example.com",
};
```

## Cognitive load reduction

### Chunk related information

Group related variables, functions, and logic together. The brain processes chunks, not individual items.

### Use whitespace strategically

Separate logical blocks with blank lines to create visual chunks.

```rust
// Group 1: Fetch and validate
let user = get_user(user_id)?;
validate_user(&user)?;

// Group 2: Calculate and update
let new_balance = calculate_balance(&user)?;
update_balance(user_id, new_balance)?;

// Group 3: Notify and log
send_notification(&user)?;
log_balance_update(user_id, new_balance);
```

### Maintain consistent patterns

If you solve a problem one way, solve similar problems the same way throughout the codebase.

### Avoid surprising behavior

Functions should do exactly what their names suggest, nothing more, nothing less.

## Anti-patterns to avoid

### God objects/functions
No single function/struct should do everything.

### Magic numbers
```rust
// DON'T
if retries > 3 { ... }

// DO
const MAX_RETRIES: u32 = 3;
if retries > MAX_RETRIES { ... }
```

### Premature optimization
Make it work, make it right, then make it fast.

### Copy-paste programming
Extract common logic into reusable functions.

### Clever code
If it makes you feel smart, it's probably too clever. Simplify.

**DON'T:**
```rust
let result = items.iter().fold(HashMap::new(), |mut acc, x| {
    *acc.entry(x.category).or_insert(vec![]).push(x); acc
});
```

**DO:**
```rust
let mut items_by_category = HashMap::new();
for item in items {
    items_by_category
        .entry(item.category)
        .or_default()
        .push(item);
}
```
