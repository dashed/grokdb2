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

## Space and Time Complexity Analysis

### Space Complexity

**Main Data Table (Decks):**
- **Space:** O(n) where n = number of nodes
- **Reasoning:** One row per node, each storing fixed-size attributes (ID, name, timestamps)

**Closure Table (DecksClosure):**
- **Best case:** O(n) — flat hierarchy (no parent-child relationships beyond self-references)
- **Average case:** O(n × h) where h = average height — balanced trees
- **Worst case:** O(n²) — completely unbalanced tree (linked list structure)

**Detailed reasoning:**
- Each node stores paths to all its descendants
- A node at depth d with s descendants stores s closure records
- In a complete binary tree: ~O(n log n) total records
- In a degenerate tree (chain): Node 1 has n-1 descendants, Node 2 has n-2, etc. = n(n-1)/2 ≈ O(n²)

**Example calculations:**
```
Flat (3 nodes, no hierarchy):     3 rows (only self-references)
Balanced tree (7 nodes, height 3): 15 rows
Chain (7 nodes):                   28 rows (7 + 6 + 5 + 4 + 3 + 2 + 1)
```

**Indexes:**
- Primary key (ancestor, descendent): O(rows in closure table)
- Depth index: O(rows in closure table)
- Additional indexes scale with closure table size

### Time Complexity Notation

**Variables used in analysis:**
- `n` = total nodes in entire tree
- `h` = height of tree (max depth)
- `d` = number of descendants of a node
- `a` = number of ancestors of a node
- `c` = number of direct children
- `s` = number of siblings
- `k` = result set size

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

**Time Complexity:** O(log n)

**Reasoning:**
- Lookup on indexed column `descendent` with equality + depth filter
- Primary key index (ancestor, descendent) enables efficient lookup
- Returns at most 1 row (nodes have at most one direct parent)
- Index seek + constant-time depth check

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

**Time Complexity:** O(h + h log h) = O(h log h) where h = height/depth of node

**Reasoning:**
- Index scan on `descendent` column finds h rows (all ancestors)
- Sorting h rows by depth: O(h log h)
- In practice, h << n for balanced trees (h ≈ log n)
- For balanced trees: O(log n × log(log n))

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

**Time Complexity:** O(c log c) where c = number of direct children

**Reasoning:**
- Index lookup on (ancestor, depth) finds c rows
- Join with Decks table: O(c) via primary key lookups
- Sorting c results by name: O(c log c)
- With composite index on (ancestor, depth): scan is O(c)

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

**Time Complexity:** O(c) where c = number of direct children

**Reasoning:**
- Same index scan as #3 but without sorting
- Database may optimize COUNT to use index-only scan
- Must scan all c matching rows to count them

### 5. Get All Descendants

Get entire subtree under a node (depth ≥ 1):

```sql
SELECT descendent
FROM DecksClosure
WHERE ancestor = :parent_id
  AND depth >= 1
ORDER BY depth ASC;
```

**Time Complexity:** O(d log d) where d = number of descendants

**Reasoning:**
- Index scan on `ancestor` finds d rows (all descendants)
- Sorting d rows by depth: O(d log d)
- Depth filter (≥ 1) applied during scan

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

**Time Complexity:** O(d) where d = number of descendants

**Reasoning:**
- Index scan on `ancestor` column
- Must scan all d matching rows
- COUNT aggregation is O(d)
- Join may be optimized away if only counting

### 7. Check if Node is Descendant of Another

```sql
SELECT COUNT(1)
FROM DecksClosure
WHERE descendent = :maybe_descendent_id
  AND ancestor = :ancestor_id
  AND depth >= 0
LIMIT 1;
```

**Time Complexity:** O(log n)

**Reasoning:**
- Direct lookup on composite primary key (ancestor, descendent)
- Returns 0 or 1, limited by LIMIT clause
- Single index probe with equality conditions on both key columns

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

**Time Complexity:** O(d) where d = descendants of deck (worst case), O(log n) typical

**Reasoning:**
- Scan closure table for all descendants of target deck: O(d)
- For each descendant, check if card's deck_id matches: O(1) per check via hash join
- Early exit with LIMIT 1 when match found
- Best case O(1) if card is in the deck itself
- Typical case: card found in first few descendants checked

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

**Time Complexity:** O((a-1) × d) where a = ancestors of child, d = descendants of child

**Reasoning:**
- First subquery finds d descendants: O(d)
- Second subquery finds a-1 ancestors (excluding self): O(a)
- DELETE operates on (a-1) × d rows (cross product of old ancestors and child's subtree)
- Each deletion is O(1) with proper indexing
- Typical case: a ≈ h (height), so O(h × d)

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

**Time Complexity:** O(a' × d) where a' = ancestors of new parent (including itself), d = descendants of child

**Reasoning:**
- Query for child's descendants: O(d)
- Query for new parent's ancestors: O(a')
- Cartesian product generates a' × d rows
- Batch INSERT of a' × d rows: O(a' × d) with index updates
- Typical case: a' ≈ h, so O(h × d)

**Combined Move Complexity:** O(a × d + a' × d) = O((a + a') × d) ≈ O(h × d) for balanced trees

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

**Time Complexity:** O(1)

**Reasoning:**
- Primary key lookup: O(1) with hash/B-tree index
- Single row update of fixed-size data
- No cascade to closure table (structure unchanged)
- Triggers add negligible overhead for timestamp updates

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

**Time Complexity:** O(1) - same as direct update, trigger fires once per row

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

**Time Complexity:** O(d + d²) where d = descendants (including node itself)

**Reasoning:**
- Subquery finds d descendants: O(d)
- Delete d rows from Decks table: O(d)
- CASCADE delete from DecksClosure:
  - Each deleted node has ≤ d related closure rows
  - Deleting d nodes triggers up to d² closure row deletions
  - Total CASCADE: O(d²)
- Dominant term: O(d²)

**Breakdown of CASCADE deletions:**
- Node at depth 0 (root of subtree): has d closure entries (to all descendants)
- Node at depth 1: has d-k closure entries (to its descendants)
- Sum of deletions: d + (d-1) + ... + 1 = d(d+1)/2 ≈ O(d²)

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

**Time Complexity:** O(offset × log c + per_page × log c) where c = total children

**Reasoning:**
- Inner subquery: sorts c children and takes first `offset` items: O(c log c)
- Outer query: filters out offset items using NOT IN: O(offset) lookups
- Sorts remaining (c - offset) items: O((c - offset) × log(c - offset))
- Takes `per_page` items: O(per_page)
- Simplified: O(c log c) dominated by sorting
- **Note:** This is less efficient than OFFSET clause for small offsets, but more consistent across databases

**Alternative with OFFSET (simpler but varies by DB):**
```sql
SELECT dc.descendent FROM DecksClosure AS dc
INNER JOIN Decks AS d ON dc.descendent = d.deck_id
WHERE dc.ancestor = :parent_id AND dc.depth = 1
ORDER BY d.name LIMIT :per_page OFFSET :offset;
```
**Time Complexity:** O(c log c + offset) - sorts once, then skips offset rows

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

**Time Complexity:** O(s) where s = number of siblings

**Reasoning:**
- Find parent via self-join with depth=1: O(1)
- Find all children of parent: O(s + 1) (includes the node itself)
- Filter out the node itself: O(1)
- Net result: O(s)

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

**Time Complexity:** O(h) where h = height/depth of node

**Reasoning:**
- Inner subquery scans h rows (all ancestors) to find MAX: O(h)
- Outer query filters to single matching row: O(1) with index
- Database may optimize to single pass: O(h)

**Optimized alternative (simpler):**
```sql
SELECT MAX(depth) FROM DecksClosure WHERE descendent = :node_id;
```
**Time Complexity:** O(h) - scans ancestors once

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

**Time Complexity:** O(n × m) where n = total nodes, m = closure table size

**Reasoning:**
- Subquery scans closure table for all depth > 0: O(m) where m is closure size
- For each of n nodes, check if in subquery results: O(n × log m) with index
- NOT IN can be inefficient; better to use NOT EXISTS or LEFT JOIN
- Simplified: O(n + m) with proper optimization

**Optimized alternative:**
```sql
SELECT d.deck_id FROM Decks d
LEFT JOIN DecksClosure dc ON d.deck_id = dc.descendent AND dc.depth > 0
WHERE dc.descendent IS NULL;
```
**Time Complexity:** O(n + m) - single scan with hash join

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

**Time Complexity:** O(n × m) unoptimized, O(n + m) optimized

**Reasoning:** Same as Get Root Nodes, but filters on ancestor column

**Optimized alternative:**
```sql
SELECT d.deck_id FROM Decks d
LEFT JOIN DecksClosure dc ON d.deck_id = dc.ancestor AND dc.depth > 0
WHERE dc.ancestor IS NULL;
```
**Time Complexity:** O(n + m) - single pass with LEFT JOIN

## Performance Considerations

### Complexity Summary Table

| Operation | Time Complexity | Space Complexity | Notes |
|-----------|----------------|------------------|-------|
| **Storage** | - | O(n) to O(n²) | Depends on tree balance |
| Get parent | O(log n) | O(1) | Index seek |
| Get path | O(h log h) | O(h) | h = height |
| Get children | O(c log c) | O(c) | c = children count |
| Count children | O(c) | O(1) | Index scan |
| Get descendants | O(d log d) | O(d) | d = descendants |
| Count descendants | O(d) | O(1) | Index scan |
| Check relationship | O(log n) | O(1) | PK lookup |
| Move subtree | O(h × d) | O(h × d) | h ancestors, d descendants |
| Update node | O(1) | O(1) | Single row |
| Delete subtree | O(d²) | - | CASCADE effect |
| Insert node | O(1) | O(1) | Trigger adds self-ref |
| Pagination | O(c log c) | O(per_page) | c = total children |

**Key insights:**
- **Read operations:** Generally O(log n) to O(k log k) where k = result size
- **Write operations:** O(1) for simple updates, O(h × d) for moves
- **Deletions:** O(d²) due to cascade; most expensive operation
- **Best for:** Frequent reads, infrequent structure changes
- **Worst case:** Deep, unbalanced trees (h ≈ n, space → O(n²))

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

**Index Impact on Complexity:**
- Without indexes: Most queries become O(n) full table scans
- Primary key (ancestor, descendent): Enables O(1) relationship checks
- Depth index: Speeds up depth-filtered queries
- Composite indexes: Eliminate index merges, improve range queries

### Query Optimization Tips

1. **Use depth filtering**: Always include depth constraints when possible
2. **Limit results**: Use `LIMIT` for single-item queries
3. **Batch operations**: Group multiple inserts/deletes in transactions
4. **Index coverage**: Ensure indexes cover your WHERE clauses
5. **Monitor tree balance**: Rebalance if height becomes excessive (h > 2 log n)
6. **Cache frequently accessed paths**: Consider application-level caching for hot paths

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

## Comparison with Other Hierarchical Patterns

### Complexity Comparison

| Pattern | Space | Get Subtree | Move Subtree | Get Path | Notes |
|---------|-------|-------------|--------------|----------|-------|
| **Adjacency List** | O(n) | O(n²) recursive | O(1) | O(h²) | Simplest, poor performance |
| **Nested Sets** | O(n) | O(k) | O(n) | O(h) | Fast reads, expensive writes |
| **Materialized Path** | O(n×h) | O(k log k) | O(d) | O(1) | String operations, limited depth |
| **Closure Table** | O(n²) | O(d) | O(h×d) | O(h log h) | Best read perf, high storage |

**When to use each pattern:**

**Adjacency List** (parent_id column):
- Simple parent-child relationships
- Rarely need full subtree queries
- Frequent structure changes
- Minimal storage constraints

**Nested Sets** (left, right values):
- Read-heavy workloads (10:1 read/write)
- Rare structure modifications
- Need fast subtree queries
- Tree rarely changes

**Materialized Path** (path string like "/1/2/3/"):
- Known maximum depth
- Need ancestor paths frequently
- Filesystem-like structures
- Want simple queries

**Closure Table** (this pattern):
- ✅ Complex hierarchical queries
- ✅ Frequent ancestor/descendant checks
- ✅ Moderate structure changes
- ✅ Storage not a primary concern
- ✅ Need move operations

### Real-World Performance

**Example: 10,000 node tree (balanced, depth=13)**

| Operation | Adjacency | Nested Sets | Path | Closure |
|-----------|-----------|-------------|------|---------|
| Storage rows | 10,000 | 10,000 | 10,000 | ~65,000 |
| Get children | 1 query | 1 query | 1 query | 1 query |
| Get all descendants | 13 recursive | 1 query | 1 query | 1 query |
| Get full path | 13 queries | 1 query | Parse string | 1 query |
| Move subtree | 1 UPDATE | Rebuild tree | UPDATE paths | 2 queries |
| Insert node | 1 INSERT | UPDATE all right nodes | 1 INSERT | 1 INSERT |

**Closure table wins when:**
- You need O(1) or O(log n) relationship checks
- Subtree queries are common (faster than recursive approaches)
- Move operations happen but aren't dominant
- Storage cost is acceptable (6-7x for balanced trees)

**Closure table loses when:**
- Storage is severely limited (use adjacency list)
- Writes vastly outnumber reads (use adjacency list)
- Tree is extremely large and flat (millions of nodes, depth < 3)

## Summary

Closure tables provide an elegant solution for hierarchical data when:
- Read performance is critical
- Complex path queries are common
- Subtree moves are needed
- Storage space is acceptable

The pattern trades storage space for query simplicity and performance, making it ideal for applications with moderate hierarchy sizes and frequent read operations.

### Complexity Quick Reference

**Space:** O(n) best case to O(n²) worst case
**Reads:** O(log n) to O(k log k) where k = result size
**Writes:** O(1) for updates, O(h × d) for moves, O(d²) for deletes
**Sweet spot:** 1K-100K nodes, balanced trees, read-heavy workloads
