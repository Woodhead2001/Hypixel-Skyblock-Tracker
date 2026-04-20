# Craft Algorithm

## Recursive Craft Calculation

The craft planner uses recursive backtracking to determine if an item can be crafted:

```
can_craft(item, inventory, recipes):
  if item in inventory and quantity >= needed:
    return True  // Already have it
  
  for each recipe that outputs this item:
    for each ingredient in recipe:
      if can_craft(ingredient, inventory, recipes):
        continue
      else:
        break  // Can't craft this recipe
    
    if all ingredients can be crafted:
      return True
  
  return False  // Can't craft item
```

## Example

Crafting "Bookshelf" needs: 6 Planks + 3 Books

1. Check inventory for Planks ✓
2. Check inventory for Books ✗
3. Recurse: Can we craft Books?
   - Books need Paper + Leather
   - Check if we can craft Paper
   - Check if we can craft Leather
   - If both yes, we can craft this item
4. Return the craft path

## Performance

- Cache craft paths to avoid recalculation
- Handle circular dependencies (Wood → Planks → Crafting Table → ???)
- Limit recursion depth to prevent infinite loops
