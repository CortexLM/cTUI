# Paragraph Component

A multi-line text rendering component with alignment and wrapping support.

## Variants

### Single Line

**Code:**

```rust
let paragraph = Paragraph::new("Hello, World!");
```

**Render:**

```
Hello, World!       
```

### Multi-line Text

**Code:**

```rust
let text = "Line one\nLine two\nLine three";
let paragraph = Paragraph::new(text);
```

**Render:**

```
Line one       
Line two       
Line three     
```

### Text Wrapping

**Code:**

```rust
let text = "This is a long line that should wrap to fit the width";
let paragraph = Paragraph::new(text);
```

**Render:**

```
This is a long 
line that      
should wrap to 
fit the width  
               
```

### Left Aligned

**Code:**

```rust
let paragraph = Paragraph::new("Left")
    .alignment(TextAlignment::Left);
```

**Render:**

```
Left           
```

### Center Aligned

**Code:**

```rust
let paragraph = Paragraph::new("Center")
    .alignment(TextAlignment::Center);
```

**Render:**

```
    Center     
```

### Right Aligned

**Code:**

```rust
let paragraph = Paragraph::new("Right")
    .alignment(TextAlignment::Right);
```

**Render:**

```
          Right
```

