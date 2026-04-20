import { useState, useEffect } from 'react';
import { api } from '../api.js';

export function usePlayer() {
  const [player, setPlayer] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const fetchPlayer = async (username) => {
    setLoading(true);
    setError(null);
    try {
      const data = await api.getPlayerData(username);
      setPlayer(JSON.parse(data));
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  return { player, loading, error, fetchPlayer };
}
