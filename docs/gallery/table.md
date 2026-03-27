# Table Component

A tabular data display component with columns, rows, and selection.

## Variants

### Basic Table

**Code:**

```rust
let table = Table::new()
    .add_column(Column::fixed("ID", 5))
    .add_column(Column::fixed("Name", 10))
    .add_row(Row::from_strings(vec!["1", "Alice"]))
    .add_row(Row::from_strings(vec!["2", "Bob"]));
```

**Render:**

```
ID   Name           
────────────────────
1    Alice          
2    Bob            
                    
```

### With Selection

**Code:**

```rust
let table = Table::new()
    .add_column(Column::fixed("Item", 10))
    .add_row(Row::from_strings(vec!["First"]))
    .add_row(Row::from_strings(vec!["Second"]))
    .with_selected(Some(0));
```

**Render:**

```
Item        
────────────
 First      
Second      
            
```

