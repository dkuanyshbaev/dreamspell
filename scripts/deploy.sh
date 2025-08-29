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
TELOXIDE_TOKEN=your_telegram_bot_token_here
DB_LOCATION=${DB_PATH}
EOF

# Step 3.5: Create compressed deployment archive
echo -e "${YELLOW}Creating deployment archive...${NC}"
tar -czf deployment.tar.gz -C dist .
if [ $? -ne 0 ]; then
    echo -e "${RED}Failed to create deployment archive${NC}"
    exit 1
fi
echo -e "${GREEN}Deployment archive created: $(du -h deployment.tar.gz | cut -f1)${NC}"

# Step 4: Deploy to server
echo -e "${YELLOW}Deploying to server ${SERVER_HOST}...${NC}"

# Upload deployment archive
echo -e "${YELLOW}Uploading deployment archive...${NC}"
scp deployment.tar.gz ${SERVER_USER}@${SERVER_HOST}:/tmp/
if [ $? -ne 0 ]; then
    echo -e "${RED}Failed to upload deployment archive${NC}"
    exit 1
fi

# Deploy on server
ssh ${SERVER_USER}@${SERVER_HOST} << 'EOF'
    set -e  # Exit on error
    
    # Stop services and create backup
    if [ -d /srv/dreamspell ]; then
        echo "Stopping services and creating backup..."
        systemctl stop dreamspell || true
        systemctl stop dreamadmin || true  
        systemctl stop dreambot || true
        
        BACKUP_DIR="/srv/backup.$(date +%Y%m%d_%H%M%S)"
        mkdir -p "${BACKUP_DIR}"
        cp -r /srv/dreamspell "${BACKUP_DIR}/" 2>/dev/null || true
        cp -r /srv/dreamadmin "${BACKUP_DIR}/" 2>/dev/null || true
        cp -r /srv/dreambot "${BACKUP_DIR}/" 2>/dev/null || true
        echo "Backup created at ${BACKUP_DIR}"
        
        rm -rf /srv/dreamspell /srv/dreamadmin /srv/dreambot
    fi
    
    # Extract new deployment
    echo "Extracting deployment archive..."
    cd /srv
    tar -xzf /tmp/deployment.tar.gz
    if [ $? -ne 0 ]; then
        echo "Failed to extract deployment archive"
        exit 1
    fi
    
    # Set executable permissions
    chmod +x /srv/dreamspell/dreamspell
    chmod +x /srv/dreamadmin/dreamadmin
    chmod +x /srv/dreambot/dreambot
    chmod +x /srv/install-services.sh
    
    # Clean up
    rm -f /tmp/deployment.tar.gz
    
    # Start services
    echo "Starting services..."
    systemctl start dreamspell || echo "dreamspell service not installed yet"
    systemctl start dreamadmin || echo "dreamadmin service not installed yet"
    systemctl start dreambot || echo "dreambot service not installed yet"

    # Check status
    echo "Service status:"
    systemctl status dreamspell --no-pager --lines=3 || true
    systemctl status dreamadmin --no-pager --lines=3 || true
    systemctl status dreambot --no-pager --lines=3 || true
EOF

echo -e "${GREEN}Deployment completed successfully!${NC}"

# Cleanup
rm -rf dist
rm -f deployment.tar.gz

echo -e "${GREEN}Cleanup completed${NC}"
echo -e "${YELLOW}Don't forget to:${NC}"
echo "  1. Update the SECRET in /srv/dreamadmin/.env (admin password)"
echo "  2. Update the TELOXIDE_TOKEN in /srv/dreambot/.env (bot token)"
echo "  3. Ensure the database exists at ${DB_PATH}"
echo "  4. Your SSL certificates and nginx are already configured"
