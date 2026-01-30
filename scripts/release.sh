#!/bin/bash
set -e

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Get version from Cargo.toml
VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
TAG="v$VERSION"

echo -e "${GREEN}Releasing rn-run $TAG${NC}"

# Check if tag already exists
if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo -e "${RED}Error: Tag $TAG already exists${NC}"
    exit 1
fi

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
    echo -e "${RED}Error: You have uncommitted changes${NC}"
    exit 1
fi

# Ensure we're on master branch
BRANCH=$(git branch --show-current)
if [ "$BRANCH" != "master" ]; then
    echo -e "${RED}Error: Must be on master branch (currently on $BRANCH)${NC}"
    exit 1
fi

# Publish to crates.io
echo -e "${GREEN}Publishing to crates.io...${NC}"
cargo publish

# Create and push tag
echo -e "${GREEN}Creating tag $TAG...${NC}"
git tag -a "$TAG" -m "Release $TAG"
git push origin "$TAG"

# Create GitHub release with auto-generated notes
echo -e "${GREEN}Creating GitHub release...${NC}"
gh release create "$TAG" \
    --title "Release $TAG" \
    --generate-notes

echo -e "${GREEN}Released $TAG successfully!${NC}"
echo "  - Published to crates.io"
echo "  - Tag pushed to GitHub"
echo "  - GitHub release created"
