const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const crypto = require("node:crypto");
const { spawn } = require("node:child_process");

const REPOSITORY = "jedaqu/android_release_doctor";
const TEST_RUNTIME_ENV = "ANDROID_RELEASE_DOCTOR_TEST_RUNTIME_PATH";

function input(name) { const envName = "INPUT_" + name.toUpperCase().replace(/-/g, "_"); return (process.env[envName] || "").trim(); }

function setOutput(name, value) { const file = process.env.GITHUB_OUTPUT; if (!file) return; fs.appendFileSync(file, name + "=" + value + "\n", "utf8"); }

function failBeforeExecution(message) { console.error("Android Release Doctor: " + message); setOutput("exit-code", "2"); setOutput("report-path", ""); process.exit(2); }

function platformKey() {
  const osName = process.platform; const arch = process.arch;
  if (osName === "linux" && arch === "x64") return "linux-x64";
  if (osName === "linux" && arch === "arm64") return "linux-arm64";
  if (osName === "darwin" && arch === "x64") return "macos-x64";
  if (osName === "darwin" && arch === "arm64") return "macos-arm64";
  if (osName === "win32" && arch === "x64") return "windows-x64";
  if (osName === "win32" && arch === "arm64") return "windows-arm64";
  throw new Error("unsupported runner platform: " + osName + "/" + arch);
}

function manifestPath() { const root = process.env.GITHUB_ACTION_PATH || path.resolve(__dirname, ".."); return path.join(root, "runtime", "manifest.json"); }

function readManifest() {
  const file = manifestPath();
  if (!fs.existsSync(file)) throw new Error("runtime manifest is missing: " + file);
  let manifest;
  try { manifest = JSON.parse(fs.readFileSync(file, "utf8")); } catch (error) { throw new Error("runtime manifest is invalid JSON: " + error.message); }
  if (manifest.schema_version !== "1") throw new Error("unsupported runtime manifest schema");
  if (manifest.repository !== REPOSITORY) throw new Error("runtime manifest repository does not match this Action");
  return manifest;
}

function isSafeEngineVersion(value) { return /^[0-9]+\.[0-9]+\.[0-9]+(?:[-+][0-9A-Za-z.-]+)?$/.test(value); }

async function downloadVerified(url, expectedSha256, destination) {
  const response = await fetch(url, { redirect: "follow", headers: { "User-Agent": "android-release-doctor-action", "Accept": "application/octet-stream" } });
  if (!response.ok) throw new Error("runtime download failed: HTTP " + response.status + " " + response.statusText);
  const data = Buffer.from(await response.arrayBuffer());
  const actualSha256 = crypto.createHash("sha256").update(data).digest("hex");
  if (actualSha256 !== expectedSha256.toLowerCase()) throw new Error("runtime SHA-256 mismatch: expected " + expectedSha256 + ", got " + actualSha256);
  const parent = path.dirname(destination); fs.mkdirSync(parent, { recursive: true });
  const temporary = destination + ".tmp-" + process.pid + "-" + Date.now(); fs.writeFileSync(temporary, data);
  if (process.platform !== "win32") fs.chmodSync(temporary, 0o755);
  try { fs.renameSync(temporary, destination); } catch (error) { if (fs.existsSync(destination)) fs.unlinkSync(temporary); else throw error; }
}

function cacheRoot() { const preferred = process.env.RUNNER_TOOL_CACHE; if (preferred) return path.join(preferred, "android-release-doctor"); const fallback = process.env.RUNNER_TEMP || os.tmpdir(); return path.join(fallback, "android-release-doctor-cache"); }

async function resolveRuntime(engineVersion, key) {
  const override = process.env[TEST_RUNTIME_ENV];
  if (override) { const resolved = path.resolve(override); if (!fs.existsSync(resolved)) throw new Error("test runtime override does not exist: " + resolved); console.log("Android Release Doctor: using internal CI prebuilt runtime override"); return resolved; }
  const manifest = readManifest();
  const entry = manifest.engines && manifest.engines[engineVersion];
  if (!entry) throw new Error("engine version " + engineVersion + " is not present in the runtime manifest");
  const asset = entry.assets && entry.assets[key];
  if (!asset) throw new Error("runtime asset " + key + " is not present for engine " + engineVersion);
  if (entry.published !== true || !asset.sha256) throw new Error("engine runtime " + engineVersion + " is not published for this Action yet; missing immutable SHA-256 metadata");
  const fileName = path.basename(asset.file); const destination = path.join(cacheRoot(), engineVersion, key, fileName);
  if (fs.existsSync(destination)) { const existing = fs.readFileSync(destination); const existingSha256 = crypto.createHash("sha256").update(existing).digest("hex"); if (existingSha256 === asset.sha256.toLowerCase()) { if (process.platform !== "win32") fs.chmodSync(destination, 0o755); return destination; } fs.unlinkSync(destination); }
  const base = manifest.release_base_url || ("https://github.com/" + REPOSITORY + "/releases/download");
  const releaseUrl = base + "/" + encodeURIComponent(entry.release_tag) + "/" + encodeURIComponent(asset.file);
  console.log("Android Release Doctor: downloading prebuilt engine " + engineVersion + " (" + key + ")");
  await downloadVerified(releaseUrl, asset.sha256, destination);
  return destination;
}

function execute(runtimePath, args, cwd) {
  return new Promise((resolve, reject) => {
    const child = spawn(runtimePath, args, { cwd, stdio: "inherit", windowsHide: true });
    child.once("error", reject);
    child.once("close", (code, signal) => { if (typeof code === "number") resolve(code); else reject(new Error("engine process terminated by signal " + (signal || "unknown"))); });
  });
}

async function main() {
  const artifact = input("artifact"); const project = input("project"); const play = input("play") || "false";
  const playPlatform = input("play-platform") || "mobile"; const format = input("format") || "text"; const output = input("output");
  let engineVersion = input("engine-version");
  if (!artifact) failBeforeExecution("artifact input is required");
  if (output.includes("\n") || output.includes("\r")) failBeforeExecution("invalid output path; CR/LF characters are not allowed");
  if (play !== "true" && play !== "false") failBeforeExecution("invalid play input; expected true or false");
  const manifest = process.env[TEST_RUNTIME_ENV] ? null : readManifest();
  if (!engineVersion) engineVersion = (manifest && manifest.default_engine_version) || "";
  if (!engineVersion || !isSafeEngineVersion(engineVersion)) failBeforeExecution("invalid engine-version input");
  let key; try { key = platformKey(); } catch (error) { failBeforeExecution(error.message); }
  const args = []; if (project) args.push("--project", project); if (play === "true") args.push("--play", "--play-platform", playPlatform); if (format) args.push("--format", format); if (output) args.push("--output", output); args.push(artifact);
  let runtimePath;
  try { runtimePath = await resolveRuntime(engineVersion, key); } catch (error) { console.error("Android Release Doctor: " + error.message); setOutput("exit-code", "2"); setOutput("report-path", ""); process.exit(2); }
  console.log("Android Release Doctor: engine=" + engineVersion + " platform=" + key);
  let status;
  try { status = await execute(runtimePath, args, process.env.GITHUB_WORKSPACE || process.cwd()); } catch (error) { console.error("Android Release Doctor: unable to execute engine: " + error.message); setOutput("exit-code", "2"); setOutput("report-path", ""); process.exit(2); }
  setOutput("exit-code", String(status)); setOutput("report-path", output); process.exit(status);
}

main().catch((error) => { console.error("Android Release Doctor: internal Action error: " + (error.stack || error.message)); setOutput("exit-code", "2"); setOutput("report-path", ""); process.exit(2); });
