// src/hooks/usePlayerSkills.js
import { useEffect, useState } from "react";
import { getPlayerSkills } from "../services/skyblockService.js";

export function usePlayerSkills(username) {
  const [skills, setSkills] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
  if (!username) return;

  console.log("Fetching skills for:", username); // ← add this

  setLoading(true);
  setError(null);

  getPlayerSkills(username)
    .then((data) => {
      console.log("Skills response:", data); // ← add this
      setSkills(data.skills || []);
    })
    .catch((err) => {
      console.error("Skill fetch error:", err); // ← add this
      setError("Failed to load skills");
    })
    .finally(() => setLoading(false));
}, [username]);


  return { skills, loading, error };
}
