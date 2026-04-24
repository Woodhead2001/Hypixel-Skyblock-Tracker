import React, { useState } from 'react';
//import { useRecipes } from '../hooks/useRecipes.js';

export function RecipeTracker() {
//  const { recipes, refetch } = useRecipes();
  const [showForm, setShowForm] = useState(false);
  const [newRecipe, setNewRecipe] = useState({
    name: '',
    description: '',
    output_item: '',
    output_quantity: 1,
  });

  const handleSubmit = async (e) => {
    e.preventDefault();
    // TODO: Add recipe to database
    setNewRecipe({ name: '', description: '', output_item: '', output_quantity: 1 });
    setShowForm(false);
  };

  return (
    <div className="recipe-tracker">
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '1.5rem' }}>
        <h2>📖 Recipe Tracker</h2>
        <button className="btn btn-primary" onClick={() => setShowForm(!showForm)}>
          {showForm ? '✕ Close' : '+ Add Recipe'}
        </button>
      </div>

      {showForm && (
        <form onSubmit={handleSubmit} className="card" style={{ marginBottom: '1.5rem' }}>
          <input
            type="text"
            placeholder="Recipe Name"
            value={newRecipe.name}
            onChange={(e) => setNewRecipe({ ...newRecipe, name: e.target.value })}
            required
            style={{ marginBottom: '1rem', width: '100%' }}
          />
          <input
            type="text"
            placeholder="Output Item"
            value={newRecipe.output_item}
            onChange={(e) => setNewRecipe({ ...newRecipe, output_item: e.target.value })}
            required
            style={{ marginBottom: '1rem', width: '100%' }}
          />
          <input
            type="number"
            placeholder="Quantity"
            min="1"
            value={newRecipe.output_quantity}
            onChange={(e) => setNewRecipe({ ...newRecipe, output_quantity: parseInt(e.target.value) })}
            style={{ marginBottom: '1rem', width: '100%' }}
          />
          <textarea
            placeholder="Description (optional)"
            value={newRecipe.description}
            onChange={(e) => setNewRecipe({ ...newRecipe, description: e.target.value })}
            style={{ marginBottom: '1rem', width: '100%', minHeight: '80px' }}
          />
          <button type="submit" className="btn btn-primary">
            Save Recipe
          </button>
        </form>
      )}

      <div className="recipe-list">
        {recipes.length === 0 ? (
          <p style={{ color: 'var(--text-light)', textAlign: 'center', padding: '2rem' }}>
            No recipes yet. Add one to get started!
          </p>
        ) : (
          recipes.map((r) => (
            <div key={r.id} className="recipe-item">
              <div className="recipe-name">{r.name}</div>
              <div className="recipe-output">
                → {r.output_quantity}x {r.output_item}
              </div>
              {r.description && (
                <p style={{ fontSize: 'var(--font-size-sm)', color: 'var(--text-light)', marginTop: '0.5rem' }}>
                  {r.description}
                </p>
              )}
            </div>
          ))
        )}
      </div>
    </div>
  );
}
