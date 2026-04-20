import React, { useState } from 'react';

export function CraftPlanner() {
  const [targetItem, setTargetItem] = useState('');
  const [craftPath, setCraftPath] = useState(null);

  const handlePlanCraft = () => {
    // TODO: Implement craft calculator
    setCraftPath([
      { step: 1, action: 'Collect Wood', status: '✓' },
      { step: 2, action: 'Craft Planks', status: '✓' },
      { step: 3, action: 'Craft Crafting Table', status: '○' },
    ]);
  };

  return (
    <div>
      <h2>🔨 Craft Planner</h2>
      <div className="card" style={{ marginBottom: '1.5rem', maxWidth: '500px' }}>
        <div style={{ display: 'flex', gap: '0.5rem' }}>
          <input
            type="text"
            placeholder="What do you want to craft?"
            value={targetItem}
            onChange={(e) => setTargetItem(e.target.value)}
            style={{ flex: 1 }}
          />
          <button className="btn btn-primary" onClick={handlePlanCraft}>
            Plan
          </button>
        </div>
      </div>

      {craftPath && (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '0.75rem' }}>
          {craftPath.map((item, idx) => (
            <div key={idx} className="card" style={{ display: 'flex', alignItems: 'center', gap: '1rem' }}>
              <div style={{ fontSize: '1.5rem', minWidth: '2rem', textAlign: 'center' }}>{item.status}</div>
              <div style={{ flex: 1 }}>
                <div style={{ fontWeight: '600' }}>Step {item.step}</div>
                <div style={{ fontSize: 'var(--font-size-sm)', color: 'var(--text-light)' }}>{item.action}</div>
              </div>
            </div>
          ))}
        </div>
      )}

      {!craftPath && (
        <p style={{ color: 'var(--text-light)', textAlign: 'center', padding: '2rem' }}>
          Enter an item and click Plan to see the craft path
        </p>
      )}
    </div>
  );
}
