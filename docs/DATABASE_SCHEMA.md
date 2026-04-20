# Database Schema & Tables

## Tables

### recipes
- id (PK)
- name (UNIQUE)
- description
- output_item
- output_quantity
- created_at

### recipe_ingredients
- id (PK)
- recipe_id (FK)
- ingredient_name
- quantity_needed

### player_data
- id (PK)
- username (UNIQUE)
- uuid
- last_updated
- data (JSON blob)

### goals
- id (PK)
- name
- description
- item_name
- quantity_target
- is_completed
- created_at
- completed_at

### inventory
- id (PK)
- username (FK)
- item_name
- quantity
- last_updated
