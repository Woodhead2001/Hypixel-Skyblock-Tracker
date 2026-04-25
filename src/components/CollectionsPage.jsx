import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export default function CollectionsPage() {
  const [minions, setMinions] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function load() {
      const data = await invoke("get_minions");
      setMinions(data.minions);
      setLoading(false);
    }
    load();
  }, []);

  if (loading) return <p>Loading minions…</p>;

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
