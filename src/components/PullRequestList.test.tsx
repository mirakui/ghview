import { render, screen, waitFor } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { PullRequestList } from "./PullRequestList";

// Mock Tauri invoke
const mockInvoke = vi.fn();
vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

describe("PullRequestList", () => {
  beforeEach(() => {
    mockInvoke.mockReset();
  });

  it("displays loading state initially", () => {
    mockInvoke.mockImplementation(() => new Promise(() => {})); // Never resolves
    render(<PullRequestList />);
    expect(screen.getByText(/loading/i)).toBeInTheDocument();
  });

  it("displays pull requests when loaded", async () => {
    mockInvoke.mockResolvedValue([
      {
        id: 1,
        number: 123,
        title: "Fix bug in login",
        html_url: "https://github.com/owner/repo/pull/123",
        user: {
          login: "author1",
          avatar_url: "https://example.com/avatar1.png",
        },
        created_at: "2024-01-15T10:00:00Z",
        updated_at: "2024-01-16T12:00:00Z",
        repository: { full_name: "owner/repo" },
        draft: false,
        labels: [{ name: "bug", color: "d73a4a" }],
      },
      {
        id: 2,
        number: 456,
        title: "Add new feature",
        html_url: "https://github.com/owner/repo2/pull/456",
        user: {
          login: "author2",
          avatar_url: "https://example.com/avatar2.png",
        },
        created_at: "2024-01-14T09:00:00Z",
        updated_at: "2024-01-15T11:00:00Z",
        repository: { full_name: "owner/repo2" },
        draft: true,
        labels: [],
      },
    ]);

    render(<PullRequestList />);

    await waitFor(() => {
      expect(screen.getByText("Fix bug in login")).toBeInTheDocument();
      expect(screen.getByText("Add new feature")).toBeInTheDocument();
      expect(screen.getByText("owner/repo")).toBeInTheDocument();
      expect(screen.getByText("owner/repo2")).toBeInTheDocument();
      expect(screen.getByText("#123")).toBeInTheDocument();
      expect(screen.getByText("#456")).toBeInTheDocument();
    });
  });

  it("displays empty state when no pull requests", async () => {
    mockInvoke.mockResolvedValue([]);
    render(<PullRequestList />);

    await waitFor(() => {
      expect(screen.getByText(/no pull requests/i)).toBeInTheDocument();
    });
  });

  it("displays error message when fetch fails", async () => {
    mockInvoke.mockRejectedValue(new Error("Failed to fetch pull requests"));
    render(<PullRequestList />);

    await waitFor(() => {
      expect(
        screen.getByText(/failed to fetch pull requests/i)
      ).toBeInTheDocument();
    });
  });

  it("displays draft badge for draft PRs", async () => {
    mockInvoke.mockResolvedValue([
      {
        id: 1,
        number: 123,
        title: "Work in progress",
        html_url: "https://github.com/owner/repo/pull/123",
        user: { login: "author", avatar_url: "https://example.com/avatar.png" },
        created_at: "2024-01-15T10:00:00Z",
        updated_at: "2024-01-16T12:00:00Z",
        repository: { full_name: "owner/repo" },
        draft: true,
        labels: [],
      },
    ]);

    render(<PullRequestList />);

    await waitFor(() => {
      expect(screen.getByText("Draft")).toBeInTheDocument();
    });
  });
});
