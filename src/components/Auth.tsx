import { useEffect, useRef } from "react";
import type { DeviceFlowInit, AuthStatus } from "../types";
import "./Auth.css";

interface AuthProps {
  authStatus: AuthStatus | null;
  loading: boolean;
  error: string | null;
  deviceFlow: DeviceFlowInit | null;
  onStartLogin: () => void;
  onPollLogin: () => void;
  onLogout: () => void;
}

export function Auth({
  authStatus,
  loading,
  error,
  deviceFlow,
  onStartLogin,
  onPollLogin,
  onLogout,
}: AuthProps) {
  const pollIntervalRef = useRef<number | null>(null);

  useEffect(() => {
    if (deviceFlow && !authStatus?.authenticated) {
      pollIntervalRef.current = window.setInterval(
        () => {
          onPollLogin();
        },
        (deviceFlow.interval + 1) * 1000
      );
    }

    return () => {
      if (pollIntervalRef.current) {
        clearInterval(pollIntervalRef.current);
        pollIntervalRef.current = null;
      }
    };
  }, [deviceFlow, authStatus?.authenticated, onPollLogin]);

  if (loading) {
    return (
      <div className="auth-container">
        <div className="auth-loading">Checking authentication...</div>
      </div>
    );
  }

  if (authStatus?.authenticated) {
    return (
      <div className="auth-container auth-logged-in">
        <span className="auth-user">
          Logged in as <strong>{authStatus.username}</strong>
        </span>
        <button className="auth-logout-btn" onClick={onLogout}>
          Logout
        </button>
      </div>
    );
  }

  if (deviceFlow) {
    return (
      <div className="auth-container auth-device-flow">
        <div className="auth-device-flow-content">
          <h3>Sign in to GitHub</h3>
          <p>Enter this code at:</p>
          <a
            href={deviceFlow.verification_uri}
            target="_blank"
            rel="noopener noreferrer"
            className="auth-verification-link"
          >
            {deviceFlow.verification_uri}
          </a>
          <div className="auth-user-code-container">
            <span className="auth-user-code">{deviceFlow.user_code}</span>
            <button
              className="auth-copy-btn"
              onClick={() =>
                navigator.clipboard.writeText(deviceFlow.user_code)
              }
              aria-label="Copy code"
            >
              <svg
                width="16"
                height="16"
                viewBox="0 0 16 16"
                fill="currentColor"
                aria-hidden="true"
              >
                <path d="M0 6.75C0 5.784.784 5 1.75 5h1.5a.75.75 0 0 1 0 1.5h-1.5a.25.25 0 0 0-.25.25v7.5c0 .138.112.25.25.25h7.5a.25.25 0 0 0 .25-.25v-1.5a.75.75 0 0 1 1.5 0v1.5A1.75 1.75 0 0 1 9.25 16h-7.5A1.75 1.75 0 0 1 0 14.25Z" />
                <path d="M5 1.75C5 .784 5.784 0 6.75 0h7.5C15.216 0 16 .784 16 1.75v7.5A1.75 1.75 0 0 1 14.25 11h-7.5A1.75 1.75 0 0 1 5 9.25Zm1.75-.25a.25.25 0 0 0-.25.25v7.5c0 .138.112.25.25.25h7.5a.25.25 0 0 0 .25-.25v-7.5a.25.25 0 0 0-.25-.25Z" />
              </svg>
            </button>
          </div>
          <p className="auth-waiting">Waiting for authorization...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="auth-container auth-login">
      <div className="auth-login-content">
        <h2>Welcome to ghview</h2>
        <p>Sign in with GitHub to view pull requests awaiting your review.</p>
        {error && <div className="auth-error">{error}</div>}
        <button className="auth-login-btn" onClick={onStartLogin}>
          Sign in with GitHub
        </button>
      </div>
    </div>
  );
}
