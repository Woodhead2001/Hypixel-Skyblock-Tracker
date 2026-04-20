import React, { useState } from 'react';

export function Dashboard() {
  const [playerStats] = useState({
    username: 'Loading...',
    level: '--',
    playtime: '--',
    networth: '--',
  });

  return (
    <div className="dashboard">
      <div className="stat-card">
        <div className="stat-label">Player</div>
        <p className="stat-value">{playerStats.username}</p>
        <div className="stat-subtitle">Search for a player to get started</div>
      </div>

      <div className="stat-card">
        <div className="stat-label">Skyblock Level</div>
        <p className="stat-value">{playerStats.level}</p>
        <div className="stat-subtitle">Overall progression</div>
      </div>

      <div className="stat-card">
        <div className="stat-label">Playtime</div>
        <p className="stat-value">{playerStats.playtime}</p>
        <div className="stat-subtitle">Total hours played</div>
      </div>

      <div className="stat-card">
        <div className="stat-label">Net Worth</div>
        <p className="stat-value">{playerStats.networth}</p>
        <div className="stat-subtitle">Estimated value</div>
      </div>
    </div>
  );
}
