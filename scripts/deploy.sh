#!/bin/bash

# Dreamspell Deployment Script
# Deploys dreamspell, dreamadmin, and dreambot to server

set -e  # Exit on error

# Configuration
SERVER_USER="denis"
SERVER_HOST="dreamspell.ru"
SERVER_PATH="/srv"
DB_PATH="/srv/dreambase/dreambase.db"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Starting Dreamspell deployment...${NC}"

# Step 1: Build all binaries in release mode
echo -e "${YELLOW}Building release binaries...${NC}"
cargo build --release

echo -e "${GREEN}Build completed successfully${NC}"

# Step 2: Create deployment package
echo -e "${YELLOW}Creating deployment package...${NC}"
rm -rf dist
mkdir -p dist/dreamspell dist/dreamadmin dist/dreambot

# Copy dreamspell files
cp target/release/dreamspell dist/dreamspell/
cp -r apps/dreamspell/static dist/dreamspell/
cp -r apps/dreamspell/templates dist/dreamspell/

# Copy dreamadmin files
cp target/release/dreamadmin dist/dreamadmin/
cp -r apps/dreamadmin/static dist/dreamadmin/
cp -r apps/dreamadmin/templates dist/dreamadmin/

# Copy dreambot binary
cp target/release/dreambot dist/dreambot/

# Copy config directories from each app
cp -r apps/dreamspell/configs dist/dreamspell/
cp -r apps/dreamadmin/configs dist/dreamadmin/
cp -r apps/dreambot/configs dist/dreambot/

# Copy installation script
cp scripts/install-services.sh dist/

echo -e "${GREEN}Package created${NC}"

# Step 3: Create .env files
echo -e "${YELLOW}Creating environment files...${NC}"

cat > dist/dreamspell/.env << EOF
DB_LOCATION=${DB_PATH}
EOF

cat > dist/dreamadmin/.env << EOF
SECRET=your_admin_password_here
DB_LOCATION=${DB_PATH}
EOF

cat > dist/dreambot/.env << EOF
SECRET=your_bot_secret_here
DB_LOCATION=${DB_PATH}
EOF

# Step 4: Deploy to server
echo -e "${YELLOW}Deploying to server ${SERVER_HOST}...${NC}"

# Create backup of existing deployment
ssh ${SERVER_USER}@${SERVER_HOST} << 'EOF'
    if [ -d /srv/dreamspell ]; then
        echo "Creating backup of existing deployment..."
        sudo cp -r /srv/dreamspell /srv/dreamspell.backup.$(date +%Y%m%d_%H%M%S)
    fi
    if [ -d /srv/dreamadmin ]; then
        sudo cp -r /srv/dreamadmin /srv/dreamadmin.backup.$(date +%Y%m%d_%H%M%S)
    fi
    if [ -d /srv/dreambot ]; then
        sudo cp -r /srv/dreambot /srv/dreambot.backup.$(date +%Y%m%d_%H%M%S)
    fi
EOF

# Upload new files
scp -r dist/dreamspell ${SERVER_USER}@${SERVER_HOST}:/tmp/
scp -r dist/dreamadmin ${SERVER_USER}@${SERVER_HOST}:/tmp/
scp -r dist/dreambot ${SERVER_USER}@${SERVER_HOST}:/tmp/

# Move files to /srv and set permissions
ssh ${SERVER_USER}@${SERVER_HOST} << 'EOF'
    echo "Installing new deployment..."

    # Stop services
    sudo systemctl stop dreamspell || true
    sudo systemctl stop dreamadmin || true
    sudo systemctl stop dreambot || true

    # Move files
    sudo rm -rf /srv/dreamspell /srv/dreamadmin /srv/dreambot
    sudo mv /tmp/dreamspell /srv/
    sudo mv /tmp/dreamadmin /srv/
    sudo mv /tmp/dreambot /srv/

    # Set permissions
    sudo chown -R denis:denis /srv/dreamspell
    sudo chown -R denis:denis /srv/dreamadmin
    sudo chown -R denis:denis /srv/dreambot

    sudo chmod +x /srv/dreamspell/dreamspell
    sudo chmod +x /srv/dreamadmin/dreamadmin
    sudo chmod +x /srv/dreambot/dreambot

    # Start services
    sudo systemctl start dreamspell
    sudo systemctl start dreamadmin
    sudo systemctl start dreambot

    # Check status
    sudo systemctl status dreamspell --no-pager
    sudo systemctl status dreamadmin --no-pager
    sudo systemctl status dreambot --no-pager
EOF

echo -e "${GREEN}Deployment completed successfully!${NC}"

# Cleanup
rm -rf dist

echo -e "${GREEN}Cleanup completed${NC}"
echo -e "${YELLOW}Don't forget to:${NC}"
echo "  1. Update the SECRET values in the .env files on the server (first time only)"
echo "  2. Ensure the database exists at ${DB_PATH}"
echo "  3. Your SSL certificates and nginx are already configured"
