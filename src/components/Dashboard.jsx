import React, { useState, useEffect } from 'react';

export function Dashboard() {
  const [playerStats, setPlayerStats] = useState({
    username: 'No player selected',
    level: '--',
    playtime: '--',
    networth: '--',
  });
  const [topSkills, setTopSkills] = useState([]);

  useEffect(() => {
    // Try to load the most recent player data from localStorage
    const recentPlayer = localStorage.getItem('recentPlayer');
    if (recentPlayer) {
      try {
        const data = JSON.parse(recentPlayer);
        setPlayerStats({
          username: data.username || 'Unknown',
          level: '--',
          playtime: '--',
          networth: '--',
        });
        
        // Extract top 3 skills
        if (data.skills) {
          const skillEntries = Object.entries(data.skills)
            .sort(([, a], [, b]) => b - a)
            .slice(0, 3)
            .map(([name, level]) => ({ name, level }));
          setTopSkills(skillEntries);
        }
      } catch (e) {
        console.error('Error loading player data:', e);
      }
    }
  }, []);

  return (
    <div className="dashboard">
      <div className="stat-card">
        <div className="stat-label">Current Player</div>
        <p className="stat-value" style={{ fontSize: 'var(--font-size-xl)' }}>{playerStats.username}</p>
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

      {topSkills.length > 0 && (
        <>
          <h3 style={{ gridColumn: '1 / -1', marginTop: '1rem' }}>📚 Top Skills</h3>
          {topSkills.map((skill, idx) => (
            <div key={skill.name} className="stat-card">
              <div className="stat-label">#{idx + 1} Skill</div>
              <p className="stat-value">{skill.level}</p>
              <div className="stat-subtitle" style={{ textTransform: 'capitalize' }}>{skill.name}</div>
            </div>
          ))}
        </>
      )}
    </div>
  );
}
