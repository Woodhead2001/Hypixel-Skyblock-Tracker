import React, { useState } from 'react';

export function PlayerSearch() {
  const [username, setUsername] = useState('');
  const [searching, setSearching] = useState(false);
  const [playerData, setPlayerData] = useState(null);
  const [error, setError] = useState('');

  const handleSearch = async (e) => {
    e.preventDefault();
    setSearching(true);
    setError('');
    try {
      // TODO: Fetch from Hypixel API
      setPlayerData({ username, uuid: 'loading...' });
    } catch (err) {
      setError('Failed to find player. Check the username and try again.');
    } finally {
      setSearching(false);
    }
  };

  return (
    <div>
      <h2>🔍 Search Player</h2>
      <form onSubmit={handleSearch} className="card" style={{ marginBottom: '2rem', maxWidth: '500px' }}>
        <div style={{ display: 'flex', gap: '0.5rem', marginBottom: '1rem' }}>
          <input
            type="text"
            placeholder="Enter Hypixel username"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            required
            style={{ flex: 1 }}
          />
          <button type="submit" className="btn btn-primary" disabled={searching}>
            {searching ? '⏳' : '🔎'} {searching ? 'Searching...' : 'Search'}
          </button>
        </div>
        {error && <p style={{ color: 'var(--primary-blue)', fontSize: 'var(--font-size-sm)' }}>⚠️ {error}</p>}
      </form>

      {playerData && (
        <div className="card">
          <h3>{playerData.username}</h3>
          <p style={{ color: 'var(--text-light)' }}>UUID: {playerData.uuid}</p>
          <p style={{ fontSize: 'var(--font-size-sm)', color: 'var(--text-light)', marginTop: '1rem' }}>
            🔄 Player data will appear here once Hypixel API integration is complete
          </p>
        </div>
      )}
    </div>
  );
}
