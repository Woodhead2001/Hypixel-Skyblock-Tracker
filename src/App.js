import React, { useState, useEffect } from 'react';
import './styles/theme.css';
import './styles/layout.css';
import './styles/components.css';

import Dashboard from "./components/Dashboard.jsx";
import CollectionsPage from './components/CollectionsPage.jsx';

import { ProfileProvider } from "./contexts/ProfileContext.jsx";
import ProfileSelector from "./components/ProfileSelector.jsx";

function App() {
  const [currentPage, setCurrentPage] = useState('dashboard');
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    setLoading(false);
  }, []);

  const renderPage = () => {
    switch (currentPage) {
      case 'dashboard':
        return <Dashboard />;
      case 'collections':
        return <CollectionsPage />;
      default:
        return <Dashboard />;
    }
  };

  return (
    <ProfileProvider>
      <div className="App">
        <div className="app-header">
              <h1>⛏️ SkyTracker</h1>
          </div>
        <div className="app-layout">
          <aside className="app-sidebar">
            <nav className="app-nav">
              <button className={`nav-item ${currentPage === 'dashboard' ? 'active' : ''}`} onClick={() => setCurrentPage('dashboard')}>📊 Dashboard</button>
              <button className={`nav-item ${currentPage === 'collections' ? 'active' : ''}`} onClick={() => setCurrentPage('collections')}>🗂️ Collections</button>
            </nav>
          </aside>

          <main className="app-main">
            <div className="page">
              <ProfileSelector />
              {loading ? <p>Loading...</p> : renderPage()}
            </div>
          </main>
        </div>
      </div>
    </ProfileProvider>
  );
}

export default App;
