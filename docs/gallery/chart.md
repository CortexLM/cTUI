# Chart Component

ASCII charts for data visualization.

## Variants

### Vertical Bar Chart

**Code:**

```rust
let data = vec![
    DataPoint::new("A", 30.0),
    DataPoint::new("B", 60.0),
    DataPoint::new("C", 90.0),
    DataPoint::new("D", 45.0),
];
let chart = Chart::new().data(data);
```

**Render:**

```
          ████      
          ████      
          ████      
     ████ ████      
     ████ ████      
     ████ ████ ████ 
     ████ ████ ████ 
A    B    C    D    
```

### Horizontal Bar Chart

**Code:**

```rust
use ctui_components::BarOrientation;
let chart = Chart::new()
    .data(data)
    .orientation(BarOrientation::Horizontal);
```

**Render:**

```
ItemA                    
ItemB ████████           
ItemC █████████          
                         
                         
```

