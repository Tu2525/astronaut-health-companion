#!/usr/bin/env bash
# Run from your laptop: ./deploy/deploy-api.sh user@host
# Pulls main on the VM, builds there (avoids cross-compiling for ARM), restarts the service.
set -euo pipefail
host="${1:?usage: deploy-api.sh user@host}"
ssh "$host" 'set -e
  cd /opt/ahc
  git fetch origin main && git reset --hard origin/main
  ~/.cargo/bin/cargo build --release -p ahc-api
  sudo systemctl restart ahc-api
  sleep 2
  systemctl is-active ahc-api'
