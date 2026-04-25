import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useProfiles } from "../contexts/ProfileContext.jsx";


export default function Dashboard() {
  const { selectedProfileId } = useProfiles();
  const [skills, setSkills] = useState(null);
  const [loadingSkills, setLoadingSkills] = useState(false);
  const [error, setError] = useState(null);

  // Fetch skills when selected profile changes
  useEffect(() => {
    if (!selectedProfileId) return;

    async function fetchSkills() {
      try {
        setLoadingSkills(true);
        const data = await invoke("get_player_skills", {
          profileId: selectedProfileId,
        });
        setSkills(data.skills || {});
      } catch (err) {
        console.error(err);
        setError(String(err));
      } finally {
        setLoadingSkills(false);
      }
    }

    fetchSkills();
  }, [selectedProfileId]);

  const skillEntries = skills
    ? Object.entries(skills).sort((a, b) => b[1].level - a[1].level)
    : [];

  return (
    <div className="dashboard" style={{ padding: "2rem", color: "#fff" }}>
      <h2>Skills</h2>

      {error && <p style={{ color: "red" }}>{error}</p>}
      {loadingSkills && <p>Loading skills…</p>}

      {!loadingSkills && skills && (
        <div style={{ marginTop: "1rem" }}>
          {skillEntries.map(([name, data]) => (
            <div
              key={name}
              style={{
                marginBottom: "1rem",
                padding: "1rem",
                background: "#1b1b1b",
                borderRadius: "10px",
                border: "1px solid #333",
              }}
            >
              <strong style={{ textTransform: "capitalize", fontSize: "1.1rem" }}>
                {name}
              </strong>

              <p style={{ margin: "0.3rem 0 0.6rem 0" }}>
                Level {data.level} / 60
              </p>

              <div
                style={{
                  position: "relative",
                  height: "14px",
                  background: "linear-gradient(180deg, #1a1a1a, #111)",
                  borderRadius: "7px",
                  overflow: "hidden",
                  boxShadow: "inset 0 0 6px rgba(0,0,0,0.6)",
                }}
              >
                <div
                  style={{
                    width: `${(data.progress * 100).toFixed(1)}%`,
                    height: "100%",
                    background: "linear-gradient(90deg, #4caf50, #6bd96b)",
                    transition: "width 0.4s ease",
                  }}
                />

                <span
                  style={{
                    position: "absolute",
                    top: 0,
                    left: "50%",
                    transform: "translateX(-50%)",
                    fontSize: "10px",
                    color: "#fff",
                    opacity: 0.8,
                    pointerEvents: "none",
                  }}
                >
                  {(data.progress * 100).toFixed(1)}%
                </span>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
