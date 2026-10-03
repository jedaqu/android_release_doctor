const fs = require("node:fs");
const path = require("node:path");
const crypto = require("node:crypto");

const version = process.env.ENGINE_VERSION;
const directory = process.env.RUNTIME_ASSETS_DIR || "release-assets";

if (!version || !/^[0-9]+\.[0-9]+\.[0-9]+(?:[-+][0-9A-Za-z.-]+)?$/.test(version)) {
  console.error("ENGINE_VERSION must be a valid semantic version");
  process.exit(1);
}

const assets = {
  "linux-x64": `android-release-doctor-${version}-linux-x64`,
  "linux-arm64": `android-release-doctor-${version}-linux-arm64`,
  "macos-x64": `android-release-doctor-${version}-macos-x64`,
  "macos-arm64": `android-release-doctor-${version}-macos-arm64`,
  "windows-x64": `android-release-doctor-${version}-windows-x64.exe`,
  "windows-arm64": `android-release-doctor-${version}-windows-arm64.exe`,
};

const root = path.resolve(directory);

for (const [key, file] of Object.entries(assets)) {
  const assetPath = path.join(root, file);
  const sidecarPath = assetPath + ".sha256";

  if (!fs.existsSync(assetPath)) {
    throw new Error(`missing runtime asset for ${key}: ${file}`);
  }
  if (!fs.existsSync(sidecarPath)) {
    throw new Error(`missing SHA-256 sidecar for ${key}: ${file}.sha256`);
  }

  const expectedLine = fs.readFileSync(sidecarPath, "utf8").trim();
  const match = expectedLine.match(/^([0-9a-fA-F]{64})\s+/);
  if (!match) {
    throw new Error(`invalid SHA-256 sidecar for ${key}: ${file}.sha256`);
  }

  const actual = crypto.createHash("sha256").update(fs.readFileSync(assetPath)).digest("hex");
  const expected = match[1].toLowerCase();

  if (actual !== expected) {
    throw new Error(`SHA-256 mismatch for ${key}: expected ${expected}, got ${actual}`);
  }

  console.log(`engine asset PASS: ${key} sha256=${actual}`);
}

console.log(`all six engine assets verified for ${version}`);
