# Input Component

A single-line text input with cursor tracking and editing support.

## Variants

### Empty Input

**Code:**

```rust
let input = Input::new();
```

**Render:**

```
                    
```

### With Placeholder

**Code:**

```rust
let input = Input::new()
    .placeholder("Enter your name...");
```

**Render:**

```
Enter your name...       
```

### With Text

**Code:**

```rust
let input = Input::new().value("Hello, World!");
```

**Render:**

```
Hello, World!            
```

### Password Field

**Code:**

```rust
let input = Input::new()
    .value("secret123")
    .password(true);
```

**Render:**

```
•••••••••           
```

