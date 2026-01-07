export interface User {
  id: number;
  login: string;
  avatar_url: string;
  html_url: string;
}

export interface Label {
  id: number;
  name: string;
  color: string;
  description: string | null;
}

export interface Repository {
  id: number;
  name: string;
  full_name: string;
  html_url: string;
  owner: User;
}

export type PullRequestState = "open" | "closed";

export interface PullRequest {
  id: number;
  number: number;
  title: string;
  html_url: string;
  state: PullRequestState;
  draft: boolean;
  created_at: string;
  updated_at: string;
  merged_at: string | null;
  user: User;
  labels: Label[];
  requested_reviewers: User[];
  repository: Repository;
}

export type CheckState = "pending" | "success" | "failure" | "error";

export interface StatusCheck {
  state: CheckState;
  context: string;
  description: string | null;
  target_url: string | null;
}

export interface CheckStatus {
  state: CheckState;
  total_count: number;
  statuses: StatusCheck[];
}

export interface PullRequestWithChecks {
  pull_request: PullRequest;
  check_status: CheckStatus | null;
}

export interface AuthStatus {
  authenticated: boolean;
  username: string | null;
}

export interface DeviceFlowInit {
  user_code: string;
  verification_uri: string;
  device_code: string;
  expires_in: number;
  interval: number;
}
