import type { PullRequestWithChecks, CheckState } from "../types";
import "./PrCard.css";

interface PrCardProps {
  prWithChecks: PullRequestWithChecks;
}

function formatRelativeTime(dateString: string): string {
  const date = new Date(dateString);
  const now = new Date();
  const diffInSeconds = Math.floor((now.getTime() - date.getTime()) / 1000);

  if (diffInSeconds < 60) {
    return `${diffInSeconds} seconds ago`;
  }
  const diffInMinutes = Math.floor(diffInSeconds / 60);
  if (diffInMinutes < 60) {
    return `${diffInMinutes} minute${diffInMinutes === 1 ? "" : "s"} ago`;
  }
  const diffInHours = Math.floor(diffInMinutes / 60);
  if (diffInHours < 24) {
    return `${diffInHours} hour${diffInHours === 1 ? "" : "s"} ago`;
  }
  const diffInDays = Math.floor(diffInHours / 24);
  if (diffInDays < 30) {
    return `${diffInDays} day${diffInDays === 1 ? "" : "s"} ago`;
  }
  const diffInMonths = Math.floor(diffInDays / 30);
  return `${diffInMonths} month${diffInMonths === 1 ? "" : "s"} ago`;
}

function CheckStatusIndicator({ state }: { state: CheckState }) {
  const statusConfig: Record<CheckState, { icon: string; className: string }> =
    {
      success: { icon: "✓", className: "check-success" },
      failure: { icon: "✗", className: "check-failure" },
      pending: { icon: "○", className: "check-pending" },
      error: { icon: "!", className: "check-error" },
    };

  const config = statusConfig[state];

  return (
    <span
      className={`check-status ${config.className}`}
      data-testid={`check-status-${state}`}
      title={`CI status: ${state}`}
    >
      {config.icon}
    </span>
  );
}

export function PrCard({ prWithChecks }: PrCardProps) {
  const { pull_request: pr, check_status } = prWithChecks;

  return (
    <article className="pr-card">
      <div className="pr-card-header">
        <div className="pr-card-title-row">
          {check_status && <CheckStatusIndicator state={check_status.state} />}
          <a
            href={pr.html_url}
            target="_blank"
            rel="noopener noreferrer"
            className="pr-card-title-link"
          >
            {pr.title}
          </a>
          <span className="pr-number">#{pr.number}</span>
          {pr.draft && <span className="pr-draft-badge">Draft</span>}
        </div>
        <div className="pr-card-meta">
          <span className="pr-repo">{pr.repository.full_name}</span>
          <span className="pr-separator">•</span>
          <img
            src={pr.user.avatar_url}
            alt={`${pr.user.login}'s avatar`}
            className="pr-author-avatar"
          />
          <span className="pr-author">{pr.user.login}</span>
          <span className="pr-separator">•</span>
          <span className="pr-updated">
            updated {formatRelativeTime(pr.updated_at)}
          </span>
        </div>
      </div>

      {pr.labels.length > 0 && (
        <div className="pr-labels">
          {pr.labels.map((label) => (
            <span
              key={label.id}
              className="pr-label"
              style={{ backgroundColor: `#${label.color}` }}
            >
              {label.name}
            </span>
          ))}
        </div>
      )}

      {pr.requested_reviewers.length > 0 && (
        <div className="pr-reviewers">
          <span className="pr-reviewers-label">Reviewers:</span>
          {pr.requested_reviewers.map((reviewer) => (
            <span key={reviewer.id} className="pr-reviewer">
              <img
                src={reviewer.avatar_url}
                alt={`${reviewer.login}'s avatar`}
                className="pr-reviewer-avatar"
              />
              {reviewer.login}
            </span>
          ))}
        </div>
      )}
    </article>
  );
}
