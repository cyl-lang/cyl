#!/usr/bin/env node

import { execSync } from "child_process";
import { readFileSync, writeFileSync } from "fs";
import { createRequire } from "module";

const require = createRequire(import.meta.url);

console.log("📦 Updating package versions...");

// Run changeset version
execSync("npx changeset version", { stdio: "inherit" });

// Get the new version from package.json
const packageJson = JSON.parse(readFileSync("package.json", "utf8"));
const newVersion = packageJson.version;

console.log(`🔄 Syncing version ${newVersion} to Cargo.toml...`);

// Update Cargo.toml
const cargoToml = readFileSync("compiler/Cargo.toml", "utf8");
const updatedCargoToml = cargoToml.replace(
  /^version = ".*"/m,
  `version = "${newVersion}"`
);
writeFileSync("compiler/Cargo.toml", updatedCargoToml);

console.log("✅ Version sync complete!");
console.log("📋 Summary:");
console.log(`  - package.json: ${newVersion}`);
console.log(`  - compiler/Cargo.toml: ${newVersion}`);

// Optionally commit
const readline = require("readline");
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

rl.question("🤔 Commit these version changes? (y/N): ", (answer) => {
  if (answer.toLowerCase() === "y" || answer.toLowerCase() === "yes") {
    try {
      execSync("git add package.json compiler/Cargo.toml CHANGELOG.md", {
        stdio: "inherit",
      });
      execSync(`git commit -m "chore: bump version to v${newVersion}"`, {
        stdio: "inherit",
      });
      console.log("✅ Changes committed!");
    } catch (error) {
      console.log("❌ Commit failed:", error.message);
    }
  } else {
    console.log("⏭️  Skipping commit. Don't forget to commit manually!");
  }
  rl.close();
});
