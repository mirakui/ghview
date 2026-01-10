import { render, screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { PrList } from "./PrList";
import type { PullRequestWithChecks } from "../types";

const mockPrs: PullRequestWithChecks[] = [
  {
    pull_request: {
      id: 1,
      number: 123,
      title: "First PR",
      html_url: "https://github.com/owner/repo1/pull/123",
      state: "open",
      draft: false,
      created_at: "2024-01-01T10:00:00Z",
      updated_at: "2024-01-02T15:30:00Z",
      merged_at: null,
      user: {
        id: 1,
        login: "user1",
        avatar_url: "https://avatars.githubusercontent.com/u/1",
        html_url: "https://github.com/user1",
      },
      labels: [],
      requested_reviewers: [],
      repository: {
        id: 1,
        name: "repo1",
        full_name: "owner/repo1",
        html_url: "https://github.com/owner/repo1",
        owner: {
          id: 10,
          login: "owner",
          avatar_url: "https://avatars.githubusercontent.com/u/10",
          html_url: "https://github.com/owner",
        },
      },
    },
    check_status: null,
  },
  {
    pull_request: {
      id: 2,
      number: 456,
      title: "Second PR",
      html_url: "https://github.com/owner/repo2/pull/456",
      state: "open",
      draft: true,
      created_at: "2024-01-03T10:00:00Z",
      updated_at: "2024-01-04T15:30:00Z",
      merged_at: null,
      user: {
        id: 2,
        login: "user2",
        avatar_url: "https://avatars.githubusercontent.com/u/2",
        html_url: "https://github.com/user2",
      },
      labels: [],
      requested_reviewers: [],
      repository: {
        id: 2,
        name: "repo2",
        full_name: "owner/repo2",
        html_url: "https://github.com/owner/repo2",
        owner: {
          id: 10,
          login: "owner",
          avatar_url: "https://avatars.githubusercontent.com/u/10",
          html_url: "https://github.com/owner",
        },
      },
    },
    check_status: {
      state: "pending",
      total_count: 1,
      statuses: [],
    },
  },
];

describe("PrList", () => {
  it("renders loading state", () => {
    render(<PrList prs={[]} loading={true} error={null} />);
    expect(screen.getByText(/loading/i)).toBeInTheDocument();
  });

  it("renders error state", () => {
    render(
      <PrList prs={[]} loading={false} error="Failed to fetch pull requests" />
    );
    expect(
      screen.getByText(/failed to fetch pull requests/i)
    ).toBeInTheDocument();
  });

  it("renders empty state when no PRs", () => {
    render(<PrList prs={[]} loading={false} error={null} />);
    expect(screen.getByText(/no pull requests/i)).toBeInTheDocument();
  });

  it("renders list of PRs", () => {
    render(<PrList prs={mockPrs} loading={false} error={null} />);
    expect(screen.getByText("First PR")).toBeInTheDocument();
    expect(screen.getByText("Second PR")).toBeInTheDocument();
  });

  it("renders correct number of PR cards", () => {
    render(<PrList prs={mockPrs} loading={false} error={null} />);
    const prCards = screen.getAllByRole("article");
    expect(prCards).toHaveLength(2);
  });

  it("displays PR count in header", () => {
    render(<PrList prs={mockPrs} loading={false} error={null} />);
    expect(screen.getByText(/2 pull request/i)).toBeInTheDocument();
  });

  it("displays singular form for single PR", () => {
    render(<PrList prs={[mockPrs[0]]} loading={false} error={null} />);
    expect(screen.getByText(/1 pull request[^s]/i)).toBeInTheDocument();
  });
});
