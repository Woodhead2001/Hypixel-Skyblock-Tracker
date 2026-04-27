import React, { useEffect, useState, useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useProfiles } from "../contexts/ProfileContext.jsx";
import "../styles/components.css";

export default function CollectionsPage() {
  const { profiles, selectedProfileId } = useProfiles();
  const [collections, setCollections] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [search, setSearch] = useState("");
  const [expanded, setExpanded] = useState({});
  const [iconCache] = useState(() => new Map());

  const selectedProfile = profiles.find(
    (p) => p.profile_id === selectedProfileId
  );

  // Load collections
  useEffect(() => {
    if (!selectedProfile) return;

    async function load() {
      try {
        setLoading(true);
        setError(null);

        const data = await invoke("get_player_collections", {
          profileId: selectedProfile.profile_id,
        });

        console.log("🔍 FULL COLLECTIONS RESPONSE:", data);

        setCollections(data.collections);

        // expand all skills by default
        const initial = {};
        Object.keys(data.collections).forEach((skill) => {
          initial[skill] = true;
        });
        setExpanded(initial);
      } catch (err) {
        console.error("❌ Failed to load collections:", err);
        setError(err.message || "Failed to load collections");
      } finally {
        setLoading(false);
      }
    }

    load();
  }, [selectedProfile]);

  // Load icons for all items (top-level hook, legal)
  useEffect(() => {
    if (!collections) return;

    async function loadAllIcons() {
      for (const group of Object.values(collections)) {
        for (const item of group.items) {
          const key = item.id;

          if (iconCache.has(key)) continue;

          try {
            const res = await invoke("get_item_icon", {
              id: item.id,
              material: item.id,
            });

            if (res?.path) {
              iconCache.set(key, res.path);
            } else {
              iconCache.set(key, null);
            }
          } catch (err) {
            console.error("❌ Icon load failed:", item.id, err);
            iconCache.set(key, null);
          }
        }
      }

      // trigger re-render
      setCollections((prev) => ({ ...prev }));
    }

    loadAllIcons();
  }, [collections, iconCache]);

  // Attach icons to items
  const itemsWithIcons = useMemo(() => {
    if (!collections) return {};

    const out = {};

    for (const [skill, group] of Object.entries(collections)) {
      out[skill] = {
        ...group,
        items: group.items.map((item) => ({
          ...item,
          icon: iconCache.get(item.id) || null,
        })),
      };
    }

    return out;
  }, [collections, iconCache]);

  const filtered = useMemo(() => {
    if (!itemsWithIcons) return {};

    const q = search.toLowerCase();
    const result = {};

    for (const [skill, group] of Object.entries(itemsWithIcons)) {
      const items = group.items.filter((c) =>
        c.name.toLowerCase().includes(q)
      );

      result[skill] = { ...group, items };
    }

    return result;
  }, [itemsWithIcons, search]);

  if (loading) return <p>Loading collections…</p>;
  if (error) return <p className="error">{error}</p>;

  return (
    <div className="collections-page">
      <div className="collections-header">
        <h2>SkyBlock Collections</h2>
        <input
          className="collections-search"
          placeholder="Search collections…"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
        />
      </div>

      {Object.entries(filtered).map(([skill, group]) => {
        const isOpen = expanded[skill];

        return (
          <div key={skill} className="skill-section">
            <div
              className="skill-header"
              onClick={() =>
                setExpanded((prev) => ({ ...prev, [skill]: !isOpen }))
              }
            >
              <h3 className="skill-title">{skill}</h3>
              <div className="skill-summary">
                {group.completed_tiers} / {group.total_tiers} tiers completed
              </div>
              <div className="skill-toggle">{isOpen ? "−" : "+"}</div>
            </div>

            {isOpen && (
              <div className="collections-grid">
                {group.items.map((c) => (
                  <div
                    key={c.id}
                    className={`collection-card ${
                      c.maxed ? "collection-card-maxed" : ""
                    }`}
                  >
                    <div className="collection-name">
                      {c.icon && (
                        <img
                          src={c.icon}
                          alt={c.name}
                          className="collection-icon"
                          onError={(e) => {
                            console.error("❌ Image failed:", c.id, c.icon);
                            e.target.style.display = "none";
                          }}
                        />
                      )}
                      {c.name}
                    </div>

                    <div className="collection-tier-display">
                      <div className="tier-label">Tier</div>
                      <div className="tier-value">
                        {c.tier} / {c.max_tier}
                      </div>
                      {c.maxed && (
                        <div className="collection-badge-maxed">MAXED</div>
                      )}
                    </div>

                    {!c.maxed && (
                      <div className="collection-progress">
                        <div className="collection-progress-label">
                          {c.count.toLocaleString()} /{" "}
                          {c.next_required.toLocaleString()} (
                          {Math.floor(c.progress * 100)}%)
                        </div>

                        <div className="progress-bar">
                          <div
                            className="progress-fill"
                            style={{ width: `${c.progress * 100}%` }}
                          />
                        </div>
                      </div>
                    )}
                  </div>
                ))}
              </div>
            )}
          </div>
        );
      })}
    </div>
  );
}
