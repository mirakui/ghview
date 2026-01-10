import { Auth, PrList } from "./components";
import { useGitHubAuth, useGitHubPRs } from "./hooks";
import "./App.css";

function App() {
  const {
    authStatus,
    loading: authLoading,
    error: authError,
    deviceFlow,
    startLogin,
    pollLogin,
    logout,
  } = useGitHubAuth();

  const {
    prs,
    loading: prsLoading,
    error: prsError,
    refresh,
  } = useGitHubPRs(authStatus?.authenticated ?? false);

  return (
    <div className="app">
      <Auth
        authStatus={authStatus}
        loading={authLoading}
        error={authError}
        deviceFlow={deviceFlow}
        onStartLogin={startLogin}
        onPollLogin={pollLogin}
        onLogout={logout}
      />
      {authStatus?.authenticated && (
        <main className="main-content">
          <div className="main-header">
            <h1>Pull Requests</h1>
            <button
              className="refresh-btn"
              onClick={refresh}
              disabled={prsLoading}
            >
              {prsLoading ? "Refreshing..." : "Refresh"}
            </button>
          </div>
          <PrList prs={prs} loading={prsLoading} error={prsError} />
        </main>
      )}
    </div>
  );
}

export default App;
