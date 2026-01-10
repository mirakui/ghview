import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import { Auth } from "./Auth";

describe("Auth", () => {
  const defaultProps = {
    authStatus: null,
    loading: false,
    error: null,
    deviceFlow: null,
    onStartLogin: vi.fn(),
    onPollLogin: vi.fn(),
    onLogout: vi.fn(),
  };

  describe("when loading", () => {
    it("displays loading message", () => {
      render(<Auth {...defaultProps} loading={true} />);
      expect(screen.getByText(/checking authentication/i)).toBeInTheDocument();
    });
  });

  describe("when not authenticated", () => {
    it("displays sign in button", () => {
      render(<Auth {...defaultProps} />);
      expect(
        screen.getByRole("button", { name: /sign in with github/i })
      ).toBeInTheDocument();
    });

    it("calls onStartLogin when sign in is clicked", () => {
      const onStartLogin = vi.fn();
      render(<Auth {...defaultProps} onStartLogin={onStartLogin} />);

      fireEvent.click(
        screen.getByRole("button", { name: /sign in with github/i })
      );

      expect(onStartLogin).toHaveBeenCalled();
    });
  });

  describe("when device flow is active", () => {
    it("displays user code and verification uri", () => {
      render(
        <Auth
          {...defaultProps}
          deviceFlow={{
            user_code: "ABCD-1234",
            verification_uri: "https://github.com/login/device",
            device_code: "device123",
            expires_in: 900,
            interval: 5,
          }}
        />
      );

      expect(screen.getByText("ABCD-1234")).toBeInTheDocument();
      expect(
        screen.getByText(/github.com\/login\/device/i)
      ).toBeInTheDocument();
    });
  });

  describe("when authenticated", () => {
    it("displays username and logout button", () => {
      render(
        <Auth
          {...defaultProps}
          authStatus={{ authenticated: true, username: "testuser" }}
        />
      );

      expect(screen.getByText(/testuser/i)).toBeInTheDocument();
      expect(
        screen.getByRole("button", { name: /logout/i })
      ).toBeInTheDocument();
    });

    it("calls onLogout when logout button is clicked", () => {
      const onLogout = vi.fn();
      render(
        <Auth
          {...defaultProps}
          authStatus={{ authenticated: true, username: "testuser" }}
          onLogout={onLogout}
        />
      );

      fireEvent.click(screen.getByRole("button", { name: /logout/i }));

      expect(onLogout).toHaveBeenCalled();
    });
  });

  describe("error handling", () => {
    it("displays error message when error is present", () => {
      render(<Auth {...defaultProps} error="Failed to start device flow" />);

      expect(
        screen.getByText(/failed to start device flow/i)
      ).toBeInTheDocument();
    });
  });
});
