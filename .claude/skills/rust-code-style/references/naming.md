# Naming Conventions

## Contents
- [Never use single-letter variable names](#never-use-single-letter-variable-names)
- [Limited exceptions to the single-letter rule](#limited-exceptions-to-the-single-letter-rule)
- [Loop variables must be descriptive](#loop-variables-must-be-descriptive)
- [Iterators and temporaries need names too](#iterators-and-temporaries-need-names-too)
- [Use full, descriptive names](#use-full-descriptive-names)
- [Avoid abbreviations](#avoid-abbreviations)
- [Use pronounceable names](#use-pronounceable-names)
- [Boolean names should form questions](#boolean-names-should-form-questions)
- [Function names should be verbs](#function-names-should-be-verbs)
- [Use domain-specific language](#use-domain-specific-language)

## Never use single-letter variable names

This is a hard rule with very limited exceptions. Single-letter variables force readers to keep mental mappings in working memory, scroll back to find context, and guess meaning from usage.

**DON'T:**
```rust
let b = get_balance();
let u = fetch_user();
let c = calculate_cost(u, b);
let t = SystemTime::now();
let r = make_request();
let s = "hello";
let i = 0;
let n = items.len();
```

**DO:**
```rust
let balance = get_balance();
let user = fetch_user();
let total_cost = calculate_cost(user, balance);
let timestamp = SystemTime::now();
let response = make_request();
let greeting = "hello";
let index = 0;
let item_count = items.len();
```

## Limited exceptions to the single-letter rule

Only acceptable in these narrow contexts:

1. **Very short closures (1–2 lines) with obvious context:**
```rust
// Acceptable - x is clearly each number in the iterator
let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();

// Better - still prefer descriptive names when not obvious
let doubled: Vec<_> = numbers.iter().map(|num| num * 2).collect();
```

2. **Mathematical formulas where letters match domain notation:**
```rust
fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    (dx * dx + dy * dy).sqrt()
}
```

3. **Generic type parameters (by convention):**
```rust
struct Container<T> { value: T }
fn map<T, U>(item: T, f: impl Fn(T) -> U) -> U
```

**When in doubt, use a full name. Always.**

## Loop variables must be descriptive

**DON'T:**
```rust
for i in 0..users.len() {
    process(users[i]);
}

for (i, u) in users.iter().enumerate() {
    println!("{}: {}", i, u.name);
}
```

**DO:**
```rust
for user_index in 0..users.len() {
    process(users[user_index]);
}

for (index, user) in users.iter().enumerate() {
    println!("{}: {}", index, user.name);
}

// Or better, avoid indices when possible
for user in &users {
    process(user);
}
```

## Iterators and temporaries need names too

**DON'T:**
```rust
let x = users.iter().filter(|u| u.is_active());
let y = x.map(|u| u.email.clone());
let z: Vec<_> = y.collect();
```

**DO:**
```rust
let active_users = users.iter().filter(|user| user.is_active());
let user_emails = active_users.map(|user| user.email.clone());
let email_list: Vec<_> = user_emails.collect();

// Or chain with clear intermediate meaning
let email_list: Vec<_> = users
    .iter()
    .filter(|user| user.is_active())
    .map(|user| user.email.clone())
    .collect();
```

## Use full, descriptive names

**DON'T:**
```rust
let usr = get_u();
let amt = calc_a(usr);
let cfg = load_cfg();
let db = connect();
let req = parse_req();
let resp = mk_resp();
```

**DO:**
```rust
let user = get_user();
let total_amount = calculate_total_amount(user);
let config = load_config();
let database = connect_to_database();
let request = parse_request();
let response = create_response();
```

## Avoid abbreviations

Only use widely-known abbreviations (HTML, URL, API, ID, HTTP, JSON, XML, SQL).

**DON'T:**
```rust
proc_req()    // process_request
init_cfg()    // initialize_config
chk_val()     // check_validation
get_usr_prfl() // get_user_profile
calc_tot()    // calculate_total
fn_usr()      // find_user
```

**DO:**
```rust
process_request()
initialize_config()
check_validation()
get_user_profile()
calculate_total()
find_user()
```

## Use pronounceable names

If you can't say it in conversation with a teammate, don't use it.

**DON'T:**
```rust
let usrxfdt = ...;    // "user x f d t"?
let genymdhms = ...;  // "gen y m d h m s"?
let prcssr = ...;     // "p r c s s r"?
```

**DO:**
```rust
let user_transfer_data = ...;
let generated_timestamp = ...;
let processor = ...;
```

## Boolean names should form questions

**DON'T:**
```rust
let authenticated;
let valid;
let active;
let enabled;
let admin;
```

**DO:**
```rust
let is_authenticated;
let is_valid;
let is_active;
let is_enabled;
let is_admin;
// or
let has_permission;
let can_edit;
let should_retry;
```

## Function names should be verbs

**DON'T:**
```rust
fn user() -> User
fn data() -> Data
fn response() -> Response
fn validation() -> bool
```

**DO:**
```rust
fn get_user() -> User
fn fetch_data() -> Data
fn create_response() -> Response
fn validate_input() -> bool
```

## Use domain-specific language

Names should reflect the business domain, not implementation details.

**DON'T:**
```rust
fn insert_db_record()
fn update_cache_entry()
fn serialize_to_json()
```

**DO:**
```rust
fn save_user()
fn update_user_profile()
fn export_user_data()
```
