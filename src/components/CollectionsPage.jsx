import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useProfiles } from "../contexts/ProfileContext.jsx";

export default function CollectionsPage() {
  const { profiles, selectedProfileId } = useProfiles();
  const [minions, setMinions] = useState(null);
  const [loading, setLoading] = useState(true);

  const selectedProfile = profiles.find(
    (p) => p.profile_id === selectedProfileId
  );

  useEffect(() => {
    if (!selectedProfile) return;

    async function load() {
      try {
        const data = await invoke("get_minions", {
          cuteName: selectedProfile.cute_name,
        });

        // FIX: backend returns { minions: [...] }
        setMinions(data.minions);
      } finally {
        setLoading(false);
      }
    }

    load();
  }, [selectedProfile]);

  if (loading) return <p>Loading minions…</p>;
  if (!minions) return <p>No minion data available.</p>;

  return (
    <div className="collections-page">
      <h2>Minion Collections</h2>

      {minions.map((m) => (
        <div key={m.id} className="minion-row">
          <h3>{m.name}</h3>

          <div className="tier-grid">
            {m.tiers.map((t) => (
              <div
                key={t.tier}
                className={`tier-box ${t.owned ? "owned" : "missing"}`}
              >
                {t.tier}
              </div>
            ))}
          </div>
        </div>
      ))}
    </div>
  );
}
