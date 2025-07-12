#!/bin/bash

# cleanup-empty-files.sh
# Deletes all empty files throughout the Cyl project
# Usage: ./scripts/cleanup-empty-files.sh [--dry-run]

set -euo pipefail

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DRY_RUN=false

# Directories to exclude from cleanup
EXCLUDE_DIRS=(
    ".git"
    "node_modules"
    "target"
    ".next"
    "dist"
    "build"
    ".cargo"
    ".rustup"
)

# File patterns to exclude (even if empty)
EXCLUDE_PATTERNS=(
    "*.keep"
    ".gitkeep"
    ".npmkeep"
    ".cargo-ok"
)

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --dry-run|-n)
            DRY_RUN=true
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [--dry-run] [--help]"
            echo ""
            echo "Options:"
            echo "  --dry-run, -n    Show what would be deleted without actually deleting"
            echo "  --help, -h       Show this help message"
            echo ""
            echo "This script deletes all empty files in the Cyl project while preserving:"
            echo "  - Files in build/cache directories (node_modules, target, etc.)"
            echo "  - Placeholder files (*.keep, .gitkeep, etc.)"
            echo "  - Git and package manager metadata"
            exit 0
            ;;
        *)
            echo "Unknown option: $1" >&2
            echo "Use --help for usage information" >&2
            exit 1
            ;;
    esac
done

echo -e "${BLUE}🧹 Cyl Project Empty File Cleanup${NC}"
echo -e "${BLUE}===================================${NC}"
echo ""

if [[ "$DRY_RUN" == "true" ]]; then
    echo -e "${YELLOW}🔍 DRY RUN MODE - No files will be deleted${NC}"
else
    echo -e "${RED}⚠️  LIVE MODE - Files will be permanently deleted${NC}"
fi

echo "📁 Project root: $PROJECT_ROOT"
echo ""

# Build find command with exclusions
FIND_CMD="find \"$PROJECT_ROOT\" -type f -size 0"

# Add directory exclusions
for exclude_dir in "${EXCLUDE_DIRS[@]}"; do
    FIND_CMD="$FIND_CMD -not -path \"*/$exclude_dir/*\""
done

# Add file pattern exclusions
for pattern in "${EXCLUDE_PATTERNS[@]}"; do
    FIND_CMD="$FIND_CMD -not -name \"$pattern\""
done

echo -e "${BLUE}🔍 Searching for empty files...${NC}"

# Execute find command and store results
EMPTY_FILES=()
while IFS= read -r -d '' file; do
    EMPTY_FILES+=("$file")
done < <(eval "$FIND_CMD -print0" 2>/dev/null || true)

if [[ ${#EMPTY_FILES[@]} -eq 0 ]]; then
    echo -e "${GREEN}✅ No empty files found!${NC}"
    echo "The project is already clean."
    exit 0
fi

echo -e "${YELLOW}📋 Found ${#EMPTY_FILES[@]} empty file(s):${NC}"
echo ""

# Display found files with relative paths
for file in "${EMPTY_FILES[@]}"; do
    if [[ -n "$file" ]]; then
        relative_path="${file#$PROJECT_ROOT/}"
        echo "  📄 $relative_path"
    fi
done

echo ""

if [[ "$DRY_RUN" == "true" ]]; then
    echo -e "${YELLOW}💡 Run without --dry-run to actually delete these files${NC}"
    exit 0
fi

# Confirm deletion in live mode
echo -e "${RED}⚠️  Are you sure you want to delete these ${#EMPTY_FILES[@]} empty files? [y/N]${NC}"
read -r response

if [[ ! "$response" =~ ^[Yy]$ ]]; then
    echo -e "${BLUE}❌ Operation cancelled${NC}"
    exit 0
fi

echo ""
echo -e "${RED}🗑️  Deleting empty files...${NC}"

DELETED_COUNT=0
FAILED_COUNT=0

for file in "${EMPTY_FILES[@]}"; do
    if [[ -n "$file" && -f "$file" ]]; then
        relative_path="${file#$PROJECT_ROOT/}"
        
        if rm "$file" 2>/dev/null; then
            echo -e "${GREEN}✅ Deleted: $relative_path${NC}"
            ((DELETED_COUNT++))
        else
            echo -e "${RED}❌ Failed to delete: $relative_path${NC}"
            ((FAILED_COUNT++))
        fi
    fi
done

echo ""
echo -e "${GREEN}🎉 Cleanup complete!${NC}"
echo -e "${GREEN}   Deleted: $DELETED_COUNT files${NC}"

if [[ $FAILED_COUNT -gt 0 ]]; then
    echo -e "${RED}   Failed: $FAILED_COUNT files${NC}"
    exit 1
else
    echo -e "${BLUE}   Failed: 0 files${NC}"
fi

echo ""
echo -e "${BLUE}💡 Tip: Run 'git status' to see if any tracked files were removed${NC}"