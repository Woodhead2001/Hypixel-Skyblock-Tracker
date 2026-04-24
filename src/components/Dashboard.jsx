import React from "react";
import { usePlayerSkills } from "../hooks/usePlayerSkills";

export default function Dashboard() {
  const username = "amaxdumbidiot"; // later: make dynamic
  const { skills, loading, error } = usePlayerSkills(username);

  return (
    <div className="dashboard">
      <h2>SkyBlock Skills</h2>

      {loading && <p>Loading skills…</p>}
      {error && <p style={{ color: "red" }}>{error}</p>}

      {!loading && !error && (
        <div className="skills-grid">
          {skills.map((skill) => (
            <div key={skill.name} className="skill-card">
              <h3>{skill.name}</h3>
              <p>
                Level: {skill.level} / {skill.maxLevel}
              </p>
              <p>XP: {Math.round(skill.xp).toLocaleString()}</p>

              <div className="skill-progress">
                <div
                  className="skill-progress-bar"
                  style={{
                    width: `${Math.min(
                      (skill.level / skill.maxLevel) * 100,
                      100
                    )}%`,
                  }}
                />
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
