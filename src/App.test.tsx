import { render, screen } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import App from "./App";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn().mockImplementation((command: string) => {
    if (command === "check_auth_status") {
      return Promise.resolve({ authenticated: false, username: null });
    }
    return Promise.resolve(null);
  }),
}));

describe("App", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("displays login screen when not authenticated", async () => {
    render(<App />);
    expect(await screen.findByText("Welcome to ghview")).toBeInTheDocument();
  });

  it("displays sign in button", async () => {
    render(<App />);
    expect(await screen.findByText("Sign in with GitHub")).toBeInTheDocument();
  });
});
