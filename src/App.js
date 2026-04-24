import React, { useState, useEffect } from 'react';
import './styles/theme.css';
import './styles/layout.css';
import './styles/components.css';
import { api } from './api.js';

import { Dashboard } from './components/Dashboard.jsx';
import { RecipeTracker } from './components/RecipeTracker.jsx';
import { GoalTracker } from './components/GoalTracker.jsx';
import { InventoryView } from './components/InventoryView.jsx';
import { CraftPlanner } from './components/CraftPlanner.jsx';
import { PlayerSearch } from './components/PlayerSearch.jsx';

function App() {
  const [currentPage, setCurrentPage] = useState('dashboard');
  const [recipes, setRecipes] = useState([]);
  const [goals, setGoals] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
      setLoading(false);
  };

  const renderPage = () => {
    switch (currentPage) {
      case 'dashboard':
        return <Dashboard />;
      case 'recipes':
        return <RecipeTracker />;
      case 'goals':
        return <GoalTracker />;
      case 'inventory':
        return <InventoryView />;
      case 'craft':
        return <CraftPlanner />;
      case 'search':
        return <PlayerSearch />;
      default:
        return <Dashboard />;
    }
  };

  return (
    <div className="App">
      <div className="app-layout">
        <aside className="app-sidebar">
          <div className="app-header">
            <h1>⛏️ SkyTracker</h1>
          </div>
          <nav className="app-nav">
            <button
              className={`nav-item ${currentPage === 'dashboard' ? 'active' : ''}`}
              onClick={() => setCurrentPage('dashboard')}
            >
              📊 Dashboard
            </button>
            <button
              className={`nav-item ${currentPage === 'search' ? 'active' : ''}`}
              onClick={() => setCurrentPage('search')}
            >
              🔍 Search Player
            </button>
            <button
              className={`nav-item ${currentPage === 'inventory' ? 'active' : ''}`}
              onClick={() => setCurrentPage('inventory')}
            >
              🎒 Inventory
            </button>
            <button
              className={`nav-item ${currentPage === 'recipes' ? 'active' : ''}`}
              onClick={() => setCurrentPage('recipes')}
            >
              📖 Recipes
            </button>
            <button
              className={`nav-item ${currentPage === 'craft' ? 'active' : ''}`}
              onClick={() => setCurrentPage('craft')}
            >
              🔨 Craft Planner
            </button>
            <button
              className={`nav-item ${currentPage === 'goals' ? 'active' : ''}`}
              onClick={() => setCurrentPage('goals')}
            >
              🎯 Goals
            </button>
          </nav>
        </aside>
        <main className="app-main">
          <div className="page">
            {loading ? <p>Loading...</p> : renderPage()}
          </div>
        </main>
      </div>
    </div>
  );
}

export default App;

