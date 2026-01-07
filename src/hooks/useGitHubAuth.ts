import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { AuthStatus, DeviceFlowInit } from "../types";

interface UseGitHubAuthReturn {
  authStatus: AuthStatus | null;
  loading: boolean;
  error: string | null;
  deviceFlow: DeviceFlowInit | null;
  startLogin: () => Promise<void>;
  pollLogin: () => Promise<void>;
  logout: () => Promise<void>;
}

export function useGitHubAuth(): UseGitHubAuthReturn {
  const [authStatus, setAuthStatus] = useState<AuthStatus | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [deviceFlow, setDeviceFlow] = useState<DeviceFlowInit | null>(null);

  const checkAuth = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const status = await invoke<AuthStatus>("check_auth_status");
      setAuthStatus(status);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
      setAuthStatus({ authenticated: false, username: null });
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    checkAuth();
  }, [checkAuth]);

  const startLogin = useCallback(async () => {
    try {
      setError(null);
      const flow = await invoke<DeviceFlowInit>("start_device_flow");
      setDeviceFlow(flow);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    }
  }, []);

  const pollLogin = useCallback(async () => {
    if (!deviceFlow) return;

    try {
      setError(null);
      const status = await invoke<AuthStatus>("poll_device_flow", {
        deviceCode: deviceFlow.device_code,
      });
      if (status.authenticated) {
        setAuthStatus(status);
        setDeviceFlow(null);
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      if (!errorMessage.includes("authorization_pending")) {
        setError(errorMessage);
      }
    }
  }, [deviceFlow]);

  const logout = useCallback(async () => {
    try {
      setError(null);
      await invoke("logout");
      setAuthStatus({ authenticated: false, username: null });
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    }
  }, []);

  return {
    authStatus,
    loading,
    error,
    deviceFlow,
    startLogin,
    pollLogin,
    logout,
  };
}
