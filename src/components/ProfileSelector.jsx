import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useProfiles } from "../contexts/ProfileContext";
import "../styles/components.css";

export default function ProfileSelector() {
  const {
    profiles,
    selectedProfileId,
    setSelectedProfileId,
    loadingProfiles,
    error,
  } = useProfiles();

  const [username, setUsername] = useState("");

  // Fetch username from Rust on component mount
  useEffect(() => {
    async function loadUsername() {
      try {
        const name = await invoke("fetch_username");
        setUsername(name);
      } catch (err) {
        console.error("Failed to fetch username:", err);
      }
    }
    loadUsername();
  }, []);

  if (loadingProfiles) return <p style={{ color: "#fff" }}>Loading profiles…</p>;
  if (error) return <p style={{ color: "red" }}>{error}</p>;
  if (!profiles.length) return <p>No profiles found.</p>;

  return (
    <div className="profile-bar">
      <span className="profile-username">
        {username ? `User: ${username}` : "Loading user…"}
      </span>
      <label className="profile-label">
        Select Profile:{" "}
        <select
          value={selectedProfileId || ""}
          onChange={(e) => setSelectedProfileId(e.target.value)}
          className="profile-select"
        >
          {profiles.map((p) => (
            <option key={p.profile_id} value={p.profile_id}>
              {p.cute_name}
            </option>
          ))}
        </select>
      </label>
    </div>
  );
}
