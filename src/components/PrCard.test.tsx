import { render, screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { PrCard } from "./PrCard";
import type { PullRequestWithChecks } from "../types";

const mockPrWithChecks: PullRequestWithChecks = {
  pull_request: {
    id: 1,
    number: 123,
    title: "Add new feature",
    html_url: "https://github.com/owner/repo/pull/123",
    state: "open",
    draft: false,
    created_at: "2024-01-01T10:00:00Z",
    updated_at: "2024-01-02T15:30:00Z",
    merged_at: null,
    user: {
      id: 1,
      login: "testuser",
      avatar_url: "https://avatars.githubusercontent.com/u/1",
      html_url: "https://github.com/testuser",
    },
    labels: [
      {
        id: 1,
        name: "bug",
        color: "d73a4a",
        description: "Something isn't working",
      },
      {
        id: 2,
        name: "enhancement",
        color: "a2eeef",
        description: "New feature or request",
      },
    ],
    requested_reviewers: [
      {
        id: 2,
        login: "reviewer1",
        avatar_url: "https://avatars.githubusercontent.com/u/2",
        html_url: "https://github.com/reviewer1",
      },
    ],
    repository: {
      id: 1,
      name: "repo",
      full_name: "owner/repo",
      html_url: "https://github.com/owner/repo",
      owner: {
        id: 3,
        login: "owner",
        avatar_url: "https://avatars.githubusercontent.com/u/3",
        html_url: "https://github.com/owner",
      },
    },
  },
  check_status: {
    state: "success",
    total_count: 2,
    statuses: [
      {
        state: "success",
        context: "ci/tests",
        description: "All tests passed",
        target_url: "https://ci.example.com/1",
      },
    ],
  },
};

describe("PrCard", () => {
  it("displays PR title", () => {
    render(<PrCard prWithChecks={mockPrWithChecks} />);
    expect(screen.getByText("Add new feature")).toBeInTheDocument();
  });

  it("displays PR number", () => {
    render(<PrCard prWithChecks={mockPrWithChecks} />);
    expect(screen.getByText("#123")).toBeInTheDocument();
  });

  it("displays repository name", () => {
    render(<PrCard prWithChecks={mockPrWithChecks} />);
    expect(screen.getByText("owner/repo")).toBeInTheDocument();
  });

  it("displays author username", () => {
    render(<PrCard prWithChecks={mockPrWithChecks} />);
    expect(screen.getByText("testuser")).toBeInTheDocument();
  });

  it("displays labels", () => {
    render(<PrCard prWithChecks={mockPrWithChecks} />);
    expect(screen.getByText("bug")).toBeInTheDocument();
    expect(screen.getByText("enhancement")).toBeInTheDocument();
  });

  it("displays requested reviewers", () => {
    render(<PrCard prWithChecks={mockPrWithChecks} />);
    expect(screen.getByText("reviewer1")).toBeInTheDocument();
  });

  it("displays check status indicator for success", () => {
    render(<PrCard prWithChecks={mockPrWithChecks} />);
    expect(screen.getByTestId("check-status-success")).toBeInTheDocument();
  });

  it("displays draft badge when PR is draft", () => {
    const draftPr: PullRequestWithChecks = {
      ...mockPrWithChecks,
      pull_request: {
        ...mockPrWithChecks.pull_request,
        draft: true,
      },
    };
    render(<PrCard prWithChecks={draftPr} />);
    expect(screen.getByText("Draft")).toBeInTheDocument();
  });

  it("displays check status indicator for failure", () => {
    const failedPr: PullRequestWithChecks = {
      ...mockPrWithChecks,
      check_status: {
        state: "failure",
        total_count: 1,
        statuses: [],
      },
    };
    render(<PrCard prWithChecks={failedPr} />);
    expect(screen.getByTestId("check-status-failure")).toBeInTheDocument();
  });

  it("displays check status indicator for pending", () => {
    const pendingPr: PullRequestWithChecks = {
      ...mockPrWithChecks,
      check_status: {
        state: "pending",
        total_count: 1,
        statuses: [],
      },
    };
    render(<PrCard prWithChecks={pendingPr} />);
    expect(screen.getByTestId("check-status-pending")).toBeInTheDocument();
  });

  it("has link to PR", () => {
    render(<PrCard prWithChecks={mockPrWithChecks} />);
    const link = screen.getByRole("link", { name: /Add new feature/i });
    expect(link).toHaveAttribute(
      "href",
      "https://github.com/owner/repo/pull/123"
    );
  });

  it("displays relative time for updated_at", () => {
    render(<PrCard prWithChecks={mockPrWithChecks} />);
    // Should show some form of time indication (e.g., "updated X ago")
    expect(screen.getByText(/updated/i)).toBeInTheDocument();
  });
});
