#!/bin/bash

# Installation script for systemd services and nginx config
# Run this on the server after deployment

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}Installing Dreamspell services...${NC}"

# Check if running as root or with sudo
if [ "$EUID" -ne 0 ]; then 
    echo -e "${RED}Please run as root or with sudo${NC}"
    exit 1
fi

# Install systemd services
echo -e "${YELLOW}Installing systemd services...${NC}"

# Copy service files
cp /srv/dreamspell/configs/dreamspell.service /etc/systemd/system/
cp /srv/dreamadmin/configs/dreamadmin.service /etc/systemd/system/
cp /srv/dreambot/configs/dreambot.service /etc/systemd/system/

# Reload systemd
systemctl daemon-reload

# Enable services
systemctl enable dreamspell.service
systemctl enable dreamadmin.service
systemctl enable dreambot.service

echo -e "${GREEN}Systemd services installed${NC}"

# Install nginx config
echo -e "${YELLOW}Installing nginx configuration...${NC}"

# Backup existing config if it exists
if [ -f /etc/nginx/sites-available/dreamspell ]; then
    cp /etc/nginx/sites-available/dreamspell /etc/nginx/sites-available/dreamspell.backup.$(date +%Y%m%d_%H%M%S)
fi

# Copy nginx config
cp /srv/dreamspell/configs/dreamspell.conf /etc/nginx/sites-available/dreamspell

# Create symlink if it doesn't exist
if [ ! -L /etc/nginx/sites-enabled/dreamspell ]; then
    ln -s /etc/nginx/sites-available/dreamspell /etc/nginx/sites-enabled/
fi

# Test nginx configuration
echo -e "${YELLOW}Testing nginx configuration...${NC}"
nginx -t

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Nginx configuration is valid${NC}"
    
    # Reload nginx
    systemctl reload nginx
    echo -e "${GREEN}Nginx reloaded${NC}"
else
    echo -e "${RED}Nginx configuration test failed!${NC}"
    echo -e "${YELLOW}Please check the configuration and fix any errors${NC}"
    exit 1
fi

# Start services
echo -e "${YELLOW}Starting services...${NC}"
systemctl start dreamspell
systemctl start dreamadmin
systemctl start dreambot

# Check status
echo -e "${YELLOW}Service status:${NC}"
systemctl status dreamspell --no-pager | head -n 3
systemctl status dreamadmin --no-pager | head -n 3
systemctl status dreambot --no-pager | head -n 3

echo -e "${GREEN}Installation completed!${NC}"
echo -e "${YELLOW}Important notes:${NC}"
echo "  1. SSL certificates are already configured via Certbot"
echo "  2. Check logs with 'journalctl -u dreamspell -f' (or dreamadmin/dreambot)"
echo "  3. Ensure database exists at /srv/dreambase/dreambase.db"
echo "  4. Services are now running - check 'systemctl status dreamspell'"