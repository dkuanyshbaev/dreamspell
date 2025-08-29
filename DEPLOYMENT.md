# Dreamspell Deployment Guide

## Overview

This guide covers deployment of the Dreamspell project, which consists of three components:
- **dreamspell** - Main web application (port 8888)
- **dreamadmin** - Admin panel (port 4444, accessible via /admin)
- **dreambot** - Telegram bot

## Server Requirements

- Ubuntu Linux with systemd
- Nginx web server
- SQLite3
- User with sudo access
- Domain name (dreamspell.ru)

## Deployment Structure

```
/srv/
├── dreamspell/
│   ├── dreamspell         (binary)
│   ├── static/            (static files)
│   ├── templates/         (HTML templates)
│   ├── configs/           (service configs)
│   └── .env               (environment variables)
├── dreamadmin/
│   ├── dreamadmin         (binary)
│   ├── static/            (static files)
│   ├── templates/         (HTML templates)
│   ├── configs/           (service configs)
│   └── .env               (environment variables)
├── dreambot/
│   ├── dreambot           (binary)
│   ├── configs/           (service configs)
│   └── .env               (environment variables)
└── dreambase/
    └── dreambase.db       (shared SQLite database)
```

## Quick Deployment

### 1. Prepare the Deployment Script

Edit `scripts/deploy.sh` and update:
```bash
SERVER_USER="your_username"
SERVER_HOST="your_server_ip_or_domain"
```

### 2. Run Deployment

```bash
./scripts/deploy.sh
```

This script will:
- Build all binaries in release mode
- Create deployment packages
- Upload to server
- Create backups of existing deployment
- Install new files
- Restart services

### 3. First-Time Server Setup

SSH into your server and run:

```bash
# Create database directory (if not exists)
sudo mkdir -p /srv/dreambase
sudo chown www-data:www-data /srv/dreambase

# Initialize database (copy your existing database or create new)
sudo cp /path/to/dreambase.db /srv/dreambase/
sudo chown www-data:www-data /srv/dreambase/dreambase.db

# Install services
sudo /srv/dreamspell/install-services.sh

# Note: SSL certificates are already configured via Certbot
```

### 4. Configure Environment Variables

Edit the `.env` files on the server:

```bash
# /srv/dreamspell/.env
DB_LOCATION=/srv/dreambase/dreambase.db

# /srv/dreamadmin/.env
SECRET=your_secure_admin_password
DB_LOCATION=/srv/dreambase/dreambase.db

# /srv/dreambot/.env
TELOXIDE_TOKEN=your_telegram_bot_token
DB_LOCATION=/srv/dreambase/dreambase.db
```

## Service Management

### Check Service Status
```bash
sudo systemctl status dreamspell
sudo systemctl status dreamadmin
sudo systemctl status dreambot
```

### View Logs
```bash
# Live logs
sudo journalctl -u dreamspell -f
sudo journalctl -u dreamadmin -f
sudo journalctl -u dreambot -f

# Last 100 lines
sudo journalctl -u dreamspell -n 100
```

### Restart Services
```bash
sudo systemctl restart dreamspell
sudo systemctl restart dreamadmin
sudo systemctl restart dreambot
```

### Stop/Start Services
```bash
sudo systemctl stop dreamspell
sudo systemctl start dreamspell
```

## Nginx Configuration

The nginx configuration handles:
- HTTPS redirection
- SSL certificates
- Path-based routing (`/admin/*` → dreamadmin)
- Proxy settings for both applications

Key routes:
- `https://dreamspell.ru/` → dreamspell app
- `https://dreamspell.ru/admin/` → dreamadmin panel
- `https://dreamspell.ru/admin/login` → admin login
- `https://dreamspell.ru/health` → dreamspell health check
- `https://dreamspell.ru/admin/health` → dreamadmin health check

## Database Management

### Backup Database
```bash
sudo sqlite3 /srv/dreambase/dreambase.db ".backup /tmp/dreambase_backup_$(date +%Y%m%d).db"
```

### Restore Database
```bash
sudo systemctl stop dreamspell dreamadmin dreambot
sudo cp /tmp/dreambase_backup.db /srv/dreambase/dreambase.db
sudo chown www-data:www-data /srv/dreambase/dreambase.db
sudo systemctl start dreamspell dreamadmin dreambot
```

## Troubleshooting

### Services Won't Start
1. Check logs: `sudo journalctl -u service_name -n 50`
2. Verify database exists and has correct permissions
3. Check .env files are present and correct
4. Ensure binaries are executable: `ls -la /srv/dreamspell/`

### 502 Bad Gateway
1. Check if services are running: `sudo systemctl status dreamspell dreamadmin`
2. Verify ports in nginx config match application ports
3. Check firewall settings

### Database Errors
1. Verify database path in .env files
2. Check database permissions: `ls -la /srv/dreambase/`
3. Test database: `sudo -u www-data sqlite3 /srv/dreambase/dreambase.db "SELECT 1;"`

### Admin Panel Not Accessible
1. Verify dreamadmin service is running
2. Check nginx is routing `/admin` correctly
3. Test directly: `curl http://localhost:4444/admin/health`

## Security Notes

1. Always use HTTPS in production
2. Keep SECRET values secure and unique
3. Regularly update SSL certificates
4. Monitor logs for suspicious activity
5. Keep database backups
6. Use firewall to restrict ports (only 80, 443 should be public)

## Monitoring

### Health Endpoints
- Main app: `https://dreamspell.ru/health`
- Admin panel: `https://dreamspell.ru/admin/health`

### System Resources
```bash
# Check memory usage
free -h

# Check disk usage
df -h

# Check process resources
htop
```

## Updates and Rollbacks

### Deploy Update
```bash
# On local machine
./scripts/deploy.sh
```

### Rollback
```bash
# On server
sudo systemctl stop dreamspell dreamadmin dreambot
sudo mv /srv/dreamspell /srv/dreamspell.failed
sudo mv /srv/dreamspell.backup.TIMESTAMP /srv/dreamspell
# Repeat for dreamadmin and dreambot
sudo systemctl start dreamspell dreamadmin dreambot
```