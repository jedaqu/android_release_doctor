const fs = require("node:fs");
const path = require("node:path");
const file = path.join(__dirname, "..", "runtime", "manifest.json");
const manifest = JSON.parse(fs.readFileSync(file, "utf8"));
const requiredKeys = ["linux-x64","linux-arm64","macos-x64","macos-arm64","windows-x64","windows-arm64"];
if (manifest.schema_version !== "1") throw new Error("manifest schema_version must be 1");
if (manifest.status !== "draft" && manifest.status !== "published") throw new Error("manifest status must be draft or published");
if (!manifest.engines || typeof manifest.engines !== "object") throw new Error("manifest engines object is missing");
if (!manifest.engines[manifest.default_engine_version]) throw new Error("default_engine_version must reference an engine entry");
for (const [version, engine] of Object.entries(manifest.engines)) {
  if (!engine.release_tag || typeof engine.published !== "boolean") throw new Error("engine " + version + " is missing release metadata");
  for (const key of requiredKeys) {
    const asset = engine.assets && engine.assets[key];
    if (!asset || !asset.file) throw new Error("engine " + version + " is missing asset " + key);
    if (engine.published) {
      if (!/^[0-9a-fA-F]{64}$/.test(asset.sha256 || "")) throw new Error("published engine " + version + " has invalid SHA-256 for " + key);
    } else if (asset.sha256 !== null) {
      throw new Error("draft engine " + version + " must keep SHA-256 unset for " + key);
    }
  }
}
if (manifest.status === "published") {
  for (const [version, engine] of Object.entries(manifest.engines)) if (!engine.published) throw new Error("published manifest contains unpublished engine " + version);
}
console.log("runtime manifest structural validation PASS");
