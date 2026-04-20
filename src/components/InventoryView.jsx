import React, { useState } from 'react';

export function InventoryView() {
  const [inventory] = useState([
    { name: 'Wood', quantity: 64, rarity: 'common' },
    { name: 'Planks', quantity: 128, rarity: 'common' },
    { name: 'Diamonds', quantity: 12, rarity: 'rare' },
  ]);

  return (
    <div>
      <h2>🎒 Inventory</h2>
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(150px, 1fr))', gap: '1rem', marginTop: '1.5rem' }}>
        {inventory.map((item, idx) => (
          <div key={idx} className="card" style={{ textAlign: 'center' }}>
            <div style={{ fontSize: '2rem', marginBottom: '0.5rem' }}>📦</div>
            <div style={{ fontWeight: '600', marginBottom: '0.25rem' }}>{item.name}</div>
            <div style={{ fontSize: 'var(--font-size-xl)', color: 'var(--primary-blue)', fontWeight: '700' }}>
              ×{item.quantity}
            </div>
          </div>
        ))}
      </div>
      {inventory.length === 0 && (
        <p style={{ color: 'var(--text-light)', textAlign: 'center', padding: '2rem' }}>
          Search for a player to see their inventory
        </p>
      )}
    </div>
  );
}
