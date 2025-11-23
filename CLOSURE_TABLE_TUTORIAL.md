# Closure Table Tutorial

## What is a Closure Table?

A closure table is a design pattern for storing and querying hierarchical data in relational databases. Unlike other approaches (adjacency list, nested sets, materialized path), closure tables store **every** ancestor-descendant relationship in the hierarchy, making complex hierarchical queries efficient and straightforward.

### Key Advantages

- **Simple queries**: No recursive CTEs needed for most operations
- **Fast reads**: All paths are pre-computed and indexed
- **Easy to maintain**: Clear insert/update/delete operations
- **Flexible**: Supports moving subtrees efficiently

### Trade-offs

- **More storage**: Stores O(n²) rows in worst case (deep hierarchy)
- **Write overhead**: More rows to maintain on insert/move operations

## Schema Structure

A closure table implementation requires two tables:

### 1. Main Data Table (Decks)

```sql
CREATE TABLE IF NOT EXISTS Decks (
    deck_id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',

    created_at INT NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INT NOT NULL DEFAULT (strftime('%s', 'now')),
    reviewed_at INT NOT NULL DEFAULT (strftime('%s', 'now')),

    CHECK (name <> '')
);
```

### 2. Closure Table (DecksClosure)

```sql
CREATE TABLE IF NOT EXISTS DecksClosure (
    ancestor INTEGER NOT NULL,
    descendent INTEGER NOT NULL,
    depth INTEGER NOT NULL,
    PRIMARY KEY(ancestor, descendent),
    FOREIGN KEY (ancestor) REFERENCES Decks(deck_id) ON DELETE CASCADE,
    FOREIGN KEY (descendent) REFERENCES Decks(deck_id) ON DELETE CASCADE
);
```

**Key columns:**
- `ancestor`: The ancestor node ID
- `descendent`: The descendant node ID (note the spelling in this implementation)
- `depth`: Distance between ancestor and descendant (0 = self-reference)

### 3. Index for Performance

```sql
CREATE INDEX IF NOT EXISTS DECKSCLOSURE_DEPTH_INDEX
ON DecksClosure (depth DESC);
```

### 4. Auto-Initialization Trigger

Every node is an ancestor/descendant of itself (depth=0):

```sql
CREATE TRIGGER IF NOT EXISTS DECKSCLOSURE_NEW_DECK_TRIGGER
AFTER INSERT ON Decks
BEGIN
    INSERT OR IGNORE INTO DecksClosure(ancestor, descendent, depth)
    VALUES (NEW.deck_id, NEW.deck_id, 0);
END;
```

## Example Hierarchy

Let's visualize a sample hierarchy:

```
Root (1)
├── Programming (2)
│   ├── Python (3)
│   └── JavaScript (4)
└── Math (5)
    └── Calculus (6)
```

The `DecksClosure` table would contain:

| ancestor | descendent | depth |
|----------|------------|-------|
| 1        | 1          | 0     |
| 1        | 2          | 1     |
| 1        | 3          | 2     |
| 1        | 4          | 2     |
| 1        | 5          | 1     |
| 1        | 6          | 2     |
| 2        | 2          | 0     |
| 2        | 3          | 1     |
| 2        | 4          | 1     |
| 3        | 3          | 0     |
| 4        | 4          | 0     |
| 5        | 5          | 0     |
| 5        | 6          | 1     |
| 6        | 6          | 0     |

## Reading Operations

### 1. Get Direct Parent

Find the immediate parent of a node (depth = 1):

```sql
SELECT ancestor
FROM DecksClosure
WHERE descendent = :child_id
  AND depth = 1
LIMIT 1;
```

**Example:** Get parent of "Python" (ID 3) → Returns 2 (Programming)

### 2. Get Full Path (Root to Node)

Get all ancestors from root to a specific node, ordered by depth:

```sql
SELECT ancestor
FROM DecksClosure
WHERE descendent = :node_id
  AND depth >= 0
ORDER BY depth DESC;
```

**Example:** Get path to "Python" (ID 3) → Returns [1, 2, 3]

### 3. Get Direct Children

Find immediate children of a node (depth = 1):

```sql
SELECT dc.descendent
FROM DecksClosure AS dc
INNER JOIN Decks AS d
  ON dc.descendent = d.deck_id
WHERE dc.ancestor = :parent_id
  AND dc.depth = 1
ORDER BY d.name COLLATE NOCASE ASC;
```

**Example:** Get children of "Programming" (ID 2) → Returns [3, 4]

### 4. Count Direct Children

```sql
SELECT COUNT(1)
FROM DecksClosure
INNER JOIN Decks
  ON DecksClosure.descendent = Decks.deck_id
WHERE ancestor = :parent_id
  AND depth = 1;
```

### 5. Get All Descendants

Get entire subtree under a node (depth ≥ 1):

```sql
SELECT descendent
FROM DecksClosure
WHERE ancestor = :parent_id
  AND depth >= 1
ORDER BY depth ASC;
```

**Example:** Get all descendants of "Root" (ID 1) → Returns [2, 5, 3, 4, 6]

### 6. Count All Descendants

```sql
SELECT COUNT(1)
FROM DecksClosure
INNER JOIN Decks
  ON DecksClosure.descendent = Decks.deck_id
WHERE ancestor = :parent_id
  AND depth >= 1;
```

### 7. Check if Node is Descendant of Another

```sql
SELECT COUNT(1)
FROM DecksClosure
WHERE descendent = :maybe_descendent_id
  AND ancestor = :ancestor_id
  AND depth >= 0
LIMIT 1;
```

Returns count ≥ 1 if relationship exists.

### 8. Cross-Table Queries: Check if Card Belongs to Deck (or Subdeck)

This demonstrates the power of closure tables for cross-table hierarchical queries:

```sql
SELECT COUNT(1)
FROM DecksClosure AS dc
INNER JOIN Cards AS c
  ON c.deck_id = dc.descendent
WHERE dc.ancestor = :deck_id
  AND c.card_id = :card_id
LIMIT 1;
```

**How it works:** Checks if a card's deck is the target deck OR any of its descendants.

## Moving Operations

### Move a Subtree to a New Parent

Moving a child (and its entire subtree) to a new parent is a two-step process:

#### Step 1: Delete Old Connections

Remove paths between the child's subtree and the old ancestors:

```sql
DELETE FROM DecksClosure

/* Select all descendants of child (including child itself) */
WHERE descendent IN (
    SELECT descendent
    FROM DecksClosure
    WHERE ancestor = :child_id
)
AND

/* Select all ancestors of child (excluding child itself) */
ancestor IN (
    SELECT ancestor
    FROM DecksClosure
    WHERE descendent = :child_id
      AND ancestor != descendent
)
AND descendent != ancestor;
```

**Example:** Moving "Python" (3) from "Programming" (2) to "Math" (5)
- Deletes paths: (1→3, depth 2) and (2→3, depth 1)

#### Step 2: Create New Connections

Insert new paths connecting the new parent's ancestors to the child's descendants:

```sql
INSERT OR IGNORE INTO DecksClosure(ancestor, descendent, depth)
SELECT p.ancestor, c.descendent, p.depth + c.depth + 1
FROM DecksClosure AS p, DecksClosure AS c
WHERE c.ancestor = :child_id
  AND p.descendent = :new_parent_id;
```

**How it works:**
- `p` represents all ancestors of the new parent (including parent itself at depth 0)
- `c` represents all descendants of the child (including child itself at depth 0)
- Cartesian product creates all necessary ancestor-descendant pairs
- `depth` is calculated as: parent's depth + child's depth + 1 (for the new edge)

**Example:** After moving "Python" (3) to "Math" (5), new paths created:
- (5→3, depth 1) - Math to Python
- (1→3, depth 2) - Root to Python (through Math)

## Updating Records

### Update Node Data

Updating node attributes doesn't affect the closure table:

```sql
UPDATE Decks
SET name = :new_name
WHERE deck_id = :deck_id;
```

### Update with Trigger

Use triggers to maintain timestamps automatically:

```sql
CREATE TRIGGER IF NOT EXISTS DECK_ON_UPDATE_TRIGGER
AFTER UPDATE OF name, description
ON Decks
BEGIN
    UPDATE Decks
    SET updated_at = strftime('%s', 'now')
    WHERE deck_id = NEW.deck_id;
END;
```

## Deletion Operations

### Delete Node and Its Subtree

Delete a node and all its descendants:

```sql
DELETE FROM Decks
WHERE deck_id IN (
    SELECT descendent
    FROM DecksClosure
    WHERE ancestor = :deck_id
);
```

**How it works:**
- Selects the target node and all descendants
- `ON DELETE CASCADE` automatically removes corresponding closure table rows
- Foreign key constraints ensure referential integrity

**Example:** Deleting "Programming" (2) also deletes "Python" (3) and "JavaScript" (4)

### Delete Single Node (Promote Children)

If you want to delete a node but keep its children (promote them to the parent):

1. Move all children to the node's parent
2. Delete the node

```sql
-- For each child, reconnect to grandparent
-- Then delete the node
DELETE FROM Decks WHERE deck_id = :node_id;
```

(Note: This implementation deletes entire subtrees; promoting children would require additional logic)

## Pagination with Closure Tables

Efficient pagination using the offset technique:

```sql
SELECT dc.descendent
FROM DecksClosure AS dc
INNER JOIN Decks AS d
  ON dc.descendent = d.deck_id
WHERE dc.descendent NOT IN (
    -- Inner query: skip first N items
    SELECT dc2.descendent
    FROM DecksClosure AS dc2
    INNER JOIN Decks AS d2
      ON dc2.descendent = d2.deck_id
    WHERE dc2.ancestor = :parent_id
      AND dc2.depth = 1
    ORDER BY d2.name COLLATE NOCASE ASC
    LIMIT :offset
)
AND dc.ancestor = :parent_id
AND dc.depth = 1
ORDER BY d.name COLLATE NOCASE ASC
LIMIT :per_page;
```

## Advanced Patterns

### 1. Get Siblings

Nodes that share the same parent:

```sql
SELECT sibling.descendent
FROM DecksClosure AS self
INNER JOIN DecksClosure AS parent
  ON self.ancestor = parent.ancestor
  AND self.depth = 1
INNER JOIN DecksClosure AS sibling
  ON parent.ancestor = sibling.ancestor
  AND sibling.depth = 1
WHERE self.descendent = :node_id
  AND sibling.descendent != :node_id;
```

### 2. Get Depth of Node

```sql
SELECT depth
FROM DecksClosure
WHERE descendent = :node_id
  AND depth = (
    SELECT MAX(depth)
    FROM DecksClosure
    WHERE descendent = :node_id
  )
LIMIT 1;
```

### 3. Get Root Nodes

Nodes with no parents (only self-reference):

```sql
SELECT DISTINCT d.deck_id
FROM Decks AS d
WHERE d.deck_id NOT IN (
    SELECT descendent
    FROM DecksClosure
    WHERE depth > 0
);
```

### 4. Get Leaf Nodes

Nodes with no children:

```sql
SELECT DISTINCT d.deck_id
FROM Decks AS d
WHERE d.deck_id NOT IN (
    SELECT ancestor
    FROM DecksClosure
    WHERE depth > 0
);
```

## Performance Considerations

### Indexes

Create indexes on frequently queried columns:

```sql
-- Primary index (already defined)
PRIMARY KEY(ancestor, descendent)

-- Depth index for filtering
CREATE INDEX idx_depth ON DecksClosure(depth DESC);

-- Consider these for specific queries
CREATE INDEX idx_descendent ON DecksClosure(descendent);
CREATE INDEX idx_ancestor_depth ON DecksClosure(ancestor, depth);
```

### Query Optimization Tips

1. **Use depth filtering**: Always include depth constraints when possible
2. **Limit results**: Use `LIMIT` for single-item queries
3. **Batch operations**: Group multiple inserts/deletes in transactions
4. **Index coverage**: Ensure indexes cover your WHERE clauses

## Common Pitfalls and Solutions

### Pitfall 1: Circular References

**Problem:** Accidentally creating cycles in the hierarchy

**Solution:** Before connecting nodes, verify the child is not an ancestor of the parent:

```sql
SELECT COUNT(1)
FROM DecksClosure
WHERE ancestor = :child_id
  AND descendent = :parent_id;
```

If count > 0, reject the operation.

### Pitfall 2: Missing Self-References

**Problem:** Forgetting to create the depth=0 self-reference

**Solution:** Use the trigger shown earlier, or always insert manually:

```sql
INSERT INTO Decks(name) VALUES ('New Deck');
INSERT INTO DecksClosure(ancestor, descendent, depth)
VALUES (last_insert_rowid(), last_insert_rowid(), 0);
```

### Pitfall 3: Incomplete Deletions

**Problem:** Orphaned closure table rows after manual deletions

**Solution:** Always use `ON DELETE CASCADE` foreign keys:

```sql
FOREIGN KEY (ancestor) REFERENCES Decks(deck_id) ON DELETE CASCADE
```

## Implementation Checklist

When implementing closure tables in your project:

- [ ] Create main data table
- [ ] Create closure table with proper foreign keys
- [ ] Add depth index
- [ ] Create trigger for self-reference on insert
- [ ] Enable foreign key constraints (`PRAGMA foreign_keys=ON`)
- [ ] Implement connect/move logic with two-step process
- [ ] Add validation to prevent circular references
- [ ] Write tests for all operations
- [ ] Consider additional indexes based on query patterns

## References

This implementation is based on patterns described in:
- *SQL Antipatterns* by Bill Karwin (Pragmatic Programmers)
- ["The Simplest Way to Do Tree-Based Queries"](http://dirtsimple.org/2010/11/simplest-way-to-do-tree-based-queries.html)

## Summary

Closure tables provide an elegant solution for hierarchical data when:
- Read performance is critical
- Complex path queries are common
- Subtree moves are needed
- Storage space is acceptable

The pattern trades storage space for query simplicity and performance, making it ideal for applications with moderate hierarchy sizes and frequent read operations.
