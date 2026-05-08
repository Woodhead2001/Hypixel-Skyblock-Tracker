import React, { useEffect, useState, useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useProfiles } from "../contexts/ProfileContext.jsx";
import "../styles/components.css";

export default function MinionsPage() {
  const { profiles, selectedProfileId } = useProfiles();
  const [minions, setMinions] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [iconCache] = useState(() => new Map());

  const selectedProfile = profiles.find(
    (p) => p.profile_id === selectedProfileId
  );

  // Load minions
  useEffect(() => {
    if (!selectedProfile) return;

    async function load() {
      try {
        setLoading(true);
        setError(null);

        const data = await invoke("get_minions", {
          profileId: selectedProfile.profile_id,
        });

        console.log("🔍 MINIONS RESPONSE:", data);

        setMinions(data.minions);
      } catch (err) {
        console.error("❌ Failed to load minions:", err);
        setError(err.message || "Failed to load minions");
      } finally {
        setLoading(false);
      }
    }

    load();
  }, [selectedProfile]);

  // Load icons
  useEffect(() => {
    if (!minions) return;

  }, [minions, iconCache]);

  const minionsWithIcons = useMemo(() => {
    if (!minions) return [];

    return minions.map((m) => ({
      ...m,
      icon: iconCache.get(m.id) || null,
    }));
  }, [minions, iconCache]);

  if (loading) return <p>Loading minions…</p>;
  if (error) return <p className="error">{error}</p>;

  return (
    <div className="page">
      <h1>Minions</h1>

      <div className="minions-list">
        {minionsWithIcons.map((m) => (
          <div key={m.id} className="minion-row">

            <div className="minion-name">
              {m.name}
            </div>

            <div className="minion-tiers">
              {m.tiers.map((owned, i) => (
                <div
                  key={i}
                  className={`tier-box ${owned ? "owned" : "missing"}`}
                >
                  {i + 1}
                </div>
              ))}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
