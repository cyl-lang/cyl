#!/bin/bash

# Update versions script
# This script runs changeset version and then syncs the version to Cargo.toml

set -e  # Exit on any error

echo "📦 Updating package versions..."

# Run changeset version to update package.json
npx changeset version

# Extract the new version from package.json
NEW_VERSION=$(node -p "require('./package.json').version")


echo "🔄 Syncing version $NEW_VERSION to Cargo.toml and docs/generator/config.json..."

# Update the version in compiler/Cargo.toml
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    sed -i '' "s/^version = \".*\"/version = \"$NEW_VERSION\"/" compiler/Cargo.toml
    # Update version in docs/generator/config.json
    sed -i '' "s/\"version\": \"[^\"]*\"/\"version\": \"$NEW_VERSION\"/" docs/generator/config.json
else
    # Linux
    sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" compiler/Cargo.toml
    sed -i "s/\"version\": \"[^\"]*\"/\"version\": \"$NEW_VERSION\"/" docs/generator/config.json
fi

echo "✅ Version sync complete!"
echo "📋 Summary:"
echo "  - package.json: $NEW_VERSION"
echo "  - compiler/Cargo.toml: $NEW_VERSION"
echo "  - docs/generator/config.json: $NEW_VERSION"

# Optionally commit the changes
read -p "🤔 Commit these version changes? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    git add package.json compiler/Cargo.toml CHANGELOG.md
    git commit -m "chore: bump version to v$NEW_VERSION"
    echo "✅ Changes committed!"
else
    echo "⏭️  Skipping commit. Don't forget to commit manually!"
fi
