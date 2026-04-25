import React from "react";
import { useProfiles } from "../contexts/ProfileContext";

export default function ProfileSelector() {
  const {
    profiles,
    selectedProfileId,
    setSelectedProfileId,
    loadingProfiles,
    error,
  } = useProfiles();

  if (loadingProfiles) return <p style={{ color: "#fff" }}>Loading profiles…</p>;
  if (error) return <p style={{ color: "red" }}>{error}</p>;
  if (!profiles.length) return <p>No profiles found.</p>;

  return (
    <div style={{ marginBottom: "1rem" }}>
      <label style={{ fontSize: "1.1rem", color: "#fff" }}>
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
