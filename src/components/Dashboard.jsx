import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export default function Dashboard() {
  const [profiles, setProfiles] = useState([]);
  const [selectedProfileId, setSelectedProfileId] = useState(null);
  const [skills, setSkills] = useState(null);
  const [loadingProfiles, setLoadingProfiles] = useState(true);
  const [loadingSkills, setLoadingSkills] = useState(false);
  const [error, setError] = useState(null);

  // Fetch profiles once
  useEffect(() => {
    async function fetchProfiles() {
      try {
        const data = await invoke("get_player_profiles");
        const profileList = data?.profiles || [];

        setProfiles(profileList);

        const active = profileList.find((p) => p.selected === true);
        if (active) {
          setSelectedProfileId(active.profile_id);
        } else if (profileList.length > 0) {
          setSelectedProfileId(profileList[0].profile_id);
        }
      } catch (err) {
        console.error(err);
        setError(String(err));
      } finally {
        setLoadingProfiles(false);
      }
    }

    fetchProfiles();
  }, []);

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

  const selectedProfile = profiles.find(
    (p) => p.profile_id === selectedProfileId
  );

  const skillEntries = skills
    ? Object.entries(skills).sort((a, b) => b[1].level - a[1].level)
    : [];

  return (
    <div className="dashboard" style={{ padding: "2rem", color: "#fff" }}>
      <h2>SkyBlock Profiles</h2>

      {loadingProfiles && <p>Loading profiles…</p>}
      {error && <p style={{ color: "red" }}>{error}</p>}

      {!loadingProfiles && !error && profiles.length > 0 && (
        <>
          {/* Profile Selector */}
          <label style={{ fontSize: "1.1rem" }}>
            Select Profile:{" "}
            <select
              value={selectedProfileId || ""}
              onChange={(e) => setSelectedProfileId(e.target.value)}
              style={{
                background: "#222",
                color: "#fff",
                padding: "0.4rem",
                borderRadius: "6px",
                border: "1px solid #444",
              }}
            >
              {profiles.map((profile) => (
                <option key={profile.profile_id} value={profile.profile_id}>
                  {profile.cute_name}
                </option>
              ))}
            </select>
          </label>

          {/* Skills */}
          <h3 style={{ marginTop: "2rem" }}>Skills</h3>
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
                  <strong
                    style={{
                      textTransform: "capitalize",
                      fontSize: "1.1rem",
                    }}
                  >
                    {name}
                  </strong>

                  <p style={{ margin: "0.3rem 0 0.6rem 0" }}>
                    Level {data.level} / 60
                  </p>

                  {/* XP BAR */}
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

          {/* Optional: Raw JSON for debugging */}
          {selectedProfile && (
            <pre
              style={{
                marginTop: "2rem",
                background: "#111",
                color: "#0f0",
                padding: "1rem",
                borderRadius: "8px",
                overflowX: "auto",
                maxHeight: "40vh",
              }}
            >
              {JSON.stringify(selectedProfile, null, 2)}
            </pre>
          )}
        </>
      )}
    </div>
  );
}
