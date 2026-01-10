import type { PullRequestWithChecks } from "../types";
import { PrCard } from "./PrCard";
import "./PrList.css";

interface PrListProps {
  prs: PullRequestWithChecks[];
  loading: boolean;
  error: string | null;
}

export function PrList({ prs, loading, error }: PrListProps) {
  if (loading) {
    return (
      <div className="pr-list-status">
        <div className="pr-list-loading">
          <span className="spinner" />
          Loading pull requests...
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="pr-list-status">
        <div className="pr-list-error">
          <span className="error-icon">⚠</span>
          {error}
        </div>
      </div>
    );
  }

  if (prs.length === 0) {
    return (
      <div className="pr-list-status">
        <div className="pr-list-empty">
          <span className="empty-icon">📋</span>
          No pull requests awaiting your review
        </div>
      </div>
    );
  }

  const prCount = prs.length;
  const prLabel = prCount === 1 ? "pull request" : "pull requests";

  return (
    <div className="pr-list">
      <header className="pr-list-header">
        <h2>
          {prCount} {prLabel} awaiting review
        </h2>
      </header>
      <div className="pr-list-items">
        {prs.map((prWithChecks) => (
          <PrCard
            key={prWithChecks.pull_request.id}
            prWithChecks={prWithChecks}
          />
        ))}
      </div>
    </div>
  );
}
