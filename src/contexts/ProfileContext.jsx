import React, { createContext, useContext, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

const ProfileContext = createContext(null);

export function ProfileProvider({ children }) {
  const [profiles, setProfiles] = useState([]);
  const [selectedProfileId, setSelectedProfileId] = useState(null);
  const [loadingProfiles, setLoadingProfiles] = useState(true);
  const [error, setError] = useState(null);

  // Load profiles once globally
  useEffect(() => {
    async function load() {
      try {
        const data = await invoke("get_player_profiles");
        const list = data?.profiles || [];

        setProfiles(list);

        const active = list.find((p) => p.selected === true);
        if (active) {
          setSelectedProfileId(active.profile_id);
        } else if (list.length > 0) {
          setSelectedProfileId(list[0].profile_id);
        }
      } catch (err) {
        console.error(err);
        setError(String(err));
      } finally {
        setLoadingProfiles(false);
      }
    }

    load();
  }, []);

  return (
    <ProfileContext.Provider
      value={{
        profiles,
        selectedProfileId,
        setSelectedProfileId,
        loadingProfiles,
        error,
      }}
    >
      {children}
    </ProfileContext.Provider>
  );
}

export function useProfiles() {
  return useContext(ProfileContext);
}
