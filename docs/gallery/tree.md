# Tree Component

Hierarchical tree view with expandable nodes.

## Variants

### Single Node

**Code:**

```rust
let tree = Tree::new().node(TreeNode::new("Root"));
```

**Render:**

```
   Root        
               
               
               
               
```

### Nested Tree (Expanded)

**Code:**

```rust
let tree = Tree::new().node(
    TreeNode::new("Root")
        .expanded(true)
        .child(TreeNode::new("Child 1"))
        .child(TreeNode::new("Child 2"))
        .child(
            TreeNode::new("Child 3")
                .expanded(true)
                .child(TreeNode::new("Grandchild"))
        )
);
```

**Render:**

```
▼  Root             
   Child 1          
   Child 2          
▼  Child 3          
   Grandchild       
```

