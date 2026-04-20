import React, { useState } from 'react';
import { useGoals } from '../hooks/useGoals.js';

export function GoalTracker() {
  const { goals, refetch } = useGoals();
  const [showForm, setShowForm] = useState(false);
  const [newGoal, setNewGoal] = useState({
    name: '',
    description: '',
    item_name: '',
    quantity_target: 0,
    is_completed: false,
  });

  const handleSubmit = async (e) => {
    e.preventDefault();
    // TODO: Add goal to database
    setNewGoal({ name: '', description: '', item_name: '', quantity_target: 0, is_completed: false });
    setShowForm(false);
  };

  return (
    <div>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '1.5rem' }}>
        <h2>🎯 Goal Tracker</h2>
        <button className="btn btn-primary" onClick={() => setShowForm(!showForm)}>
          {showForm ? '✕ Close' : '+ Add Goal'}
        </button>
      </div>

      {showForm && (
        <form onSubmit={handleSubmit} className="card" style={{ marginBottom: '1.5rem' }}>
          <input
            type="text"
            placeholder="Goal Name"
            value={newGoal.name}
            onChange={(e) => setNewGoal({ ...newGoal, name: e.target.value })}
            required
            style={{ marginBottom: '1rem', width: '100%' }}
          />
          <input
            type="text"
            placeholder="Item Name (optional)"
            value={newGoal.item_name}
            onChange={(e) => setNewGoal({ ...newGoal, item_name: e.target.value })}
            style={{ marginBottom: '1rem', width: '100%' }}
          />
          <input
            type="number"
            placeholder="Target Quantity"
            value={newGoal.quantity_target}
            onChange={(e) => setNewGoal({ ...newGoal, quantity_target: parseInt(e.target.value) })}
            style={{ marginBottom: '1rem', width: '100%' }}
          />
          <textarea
            placeholder="Description (optional)"
            value={newGoal.description}
            onChange={(e) => setNewGoal({ ...newGoal, description: e.target.value })}
            style={{ marginBottom: '1rem', width: '100%', minHeight: '80px' }}
          />
          <button type="submit" className="btn btn-primary">
            Create Goal
          </button>
        </form>
      )}

      <div className="goal-list">
        {goals.length === 0 ? (
          <p style={{ color: 'var(--text-light)', textAlign: 'center', padding: '2rem' }}>
            No goals yet. Create one to get started!
          </p>
        ) : (
          goals.map((g) => (
            <div key={g.id} className={`goal-item ${g.is_completed ? 'goal-completed' : ''}`}>
              <input type="checkbox" className="goal-checkbox" defaultChecked={g.is_completed} />
              <div className="goal-info">
                <div className="goal-name">{g.name}</div>
                {g.item_name && g.quantity_target && (
                  <div className="goal-progress">
                    📦 {g.item_name} × {g.quantity_target}
                  </div>
                )}
                {g.description && (
                  <div className="goal-progress">{g.description}</div>
                )}
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
}
