import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { PullRequestWithChecks } from "../types";

interface UseGitHubPRsReturn {
  prs: PullRequestWithChecks[];
  loading: boolean;
  error: string | null;
  refresh: () => Promise<void>;
}

export function useGitHubPRs(authenticated: boolean): UseGitHubPRsReturn {
  const [prs, setPrs] = useState<PullRequestWithChecks[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchPRs = useCallback(async () => {
    if (!authenticated) {
      setPrs([]);
      return;
    }

    try {
      setLoading(true);
      setError(null);
      const result = await invoke<PullRequestWithChecks[]>(
        "fetch_review_requested_prs"
      );
      setPrs(result);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
      setPrs([]);
    } finally {
      setLoading(false);
    }
  }, [authenticated]);

  useEffect(() => {
    fetchPRs();
  }, [fetchPRs]);

  return {
    prs,
    loading,
    error,
    refresh: fetchPRs,
  };
}
