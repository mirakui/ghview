/**
 * Desktop E2E Test using TestDriver AI
 *
 * This script launches the Tauri app and tests it using Vision AI.
 * Requires TD_API_KEY environment variable to be set.
 *
 * Usage:
 *   export TD_API_KEY="your_api_key"
 *   pnpm e2e:desktop
 */

import { spawn, ChildProcess } from "child_process";
import * as path from "path";

const PROJECT_ROOT = path.resolve(__dirname, "..");

interface TestResult {
  name: string;
  passed: boolean;
  error?: string;
}

async function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function startTauriDev(): Promise<ChildProcess> {
  console.log("Starting Tauri dev server...");

  const tauriProcess = spawn("pnpm", ["tauri", "dev"], {
    cwd: PROJECT_ROOT,
    stdio: ["ignore", "pipe", "pipe"],
    shell: true,
  });

  // Wait for app to start
  await sleep(15000);

  return tauriProcess;
}

async function runTestDriverTest(testFile: string): Promise<TestResult> {
  return new Promise((resolve) => {
    const testProcess = spawn("npx", ["testdriverai", "run", testFile], {
      cwd: PROJECT_ROOT,
      stdio: ["ignore", "pipe", "pipe"],
      shell: true,
      env: {
        ...process.env,
        TD_API_KEY: process.env.TD_API_KEY,
      },
    });

    let stdout = "";
    let stderr = "";

    testProcess.stdout?.on("data", (data) => {
      stdout += data.toString();
      console.log(data.toString());
    });

    testProcess.stderr?.on("data", (data) => {
      stderr += data.toString();
      console.error(data.toString());
    });

    testProcess.on("close", (code) => {
      resolve({
        name: testFile,
        passed: code === 0,
        error: code !== 0 ? stderr || stdout : undefined,
      });
    });
  });
}

async function main(): Promise<void> {
  if (!process.env.TD_API_KEY) {
    console.error("Error: TD_API_KEY environment variable is not set.");
    console.error("Please set it with: export TD_API_KEY='your_api_key'");
    process.exit(1);
  }

  let tauriProcess: ChildProcess | null = null;

  try {
    // Start Tauri app
    tauriProcess = await startTauriDev();

    // Run TestDriver AI tests
    const results: TestResult[] = [];

    const testFiles = [".testdriver/hello-world.yaml"];

    for (const testFile of testFiles) {
      console.log(`\nRunning test: ${testFile}`);
      const result = await runTestDriverTest(testFile);
      results.push(result);
    }

    // Print summary
    console.log("\n=== Test Results ===");
    for (const result of results) {
      const status = result.passed ? "PASSED" : "FAILED";
      console.log(`${status}: ${result.name}`);
      if (result.error) {
        console.log(`  Error: ${result.error}`);
      }
    }

    const allPassed = results.every((r) => r.passed);
    process.exit(allPassed ? 0 : 1);
  } finally {
    // Cleanup
    if (tauriProcess) {
      console.log("\nStopping Tauri app...");
      tauriProcess.kill("SIGTERM");
    }
  }
}

main().catch((error) => {
  console.error("Test runner failed:", error);
  process.exit(1);
});
