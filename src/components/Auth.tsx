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
            <div className="auth-user-code">{deviceFlow.user_code}</div>
            <button
              className="auth-copy-btn"
              onClick={() =>
                navigator.clipboard.writeText(deviceFlow.user_code)
              }
              aria-label="Copy code"
            >
              Copy
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
