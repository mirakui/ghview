import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

interface User {
  login: string;
  avatar_url: string;
}

interface Repository {
  full_name: string;
}

interface Label {
  name: string;
  color: string;
}

interface PullRequest {
  id: number;
  number: number;
  title: string;
  html_url: string;
  user: User;
  created_at: string;
  updated_at: string;
  repository: Repository;
  draft: boolean;
  labels: Label[];
}

export function PullRequestList() {
  const [pullRequests, setPullRequests] = useState<PullRequest[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchPRs = async () => {
      try {
        const prs = await invoke<PullRequest[]>("fetch_pull_requests");
        setPullRequests(prs);
      } catch (err) {
        setError(err instanceof Error ? err.message : String(err));
      } finally {
        setLoading(false);
      }
    };

    fetchPRs();
  }, []);

  if (loading) {
    return <div className="pr-loading">Loading pull requests...</div>;
  }

  if (error) {
    return <div className="pr-error">Error: {error}</div>;
  }

  if (pullRequests.length === 0) {
    return <div className="pr-empty">No pull requests awaiting your review</div>;
  }

  return (
    <div className="pr-list">
      <h2>Pull Requests Awaiting Review</h2>
      <ul>
        {pullRequests.map((pr) => (
          <li key={pr.id} className="pr-item">
            <div className="pr-header">
              <a href={pr.html_url} target="_blank" rel="noopener noreferrer" className="pr-title">
                {pr.title}
              </a>
              {pr.draft && <span className="pr-draft-badge">Draft</span>}
            </div>
            <div className="pr-meta">
              <span className="pr-repo">{pr.repository.full_name}</span>
              <span className="pr-number">#{pr.number}</span>
              <span className="pr-author">
                <img src={pr.user.avatar_url} alt={pr.user.login} className="pr-avatar" />
                {pr.user.login}
              </span>
            </div>
            {pr.labels.length > 0 && (
              <div className="pr-labels">
                {pr.labels.map((label) => (
                  <span
                    key={label.name}
                    className="pr-label"
                    style={{ backgroundColor: `#${label.color}` }}
                  >
                    {label.name}
                  </span>
                ))}
              </div>
            )}
          </li>
        ))}
      </ul>
    </div>
  );
}
