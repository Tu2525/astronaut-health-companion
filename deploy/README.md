# DEVOPS: `deploy/` and `.github/`

Owners: **@Tu2525** and **@m7mdnasrr**. Tasks: `docs/plan.md`, section DEVOPS.

**You own:** `.github/` (workflows, PR template, CODEOWNERS) and `deploy/` (everything needed to run the project somewhere other than a laptop).
**Not yours:** application code. When a build breaks because of code, the owner of that code fixes it; you fix it when the workflow itself is wrong.

## What runs where

| Thing | Where | How it gets there |
|---|---|---|
| Web app (Crew App + Ground Station) | Cloudflare Pages (`*.pages.dev`) | GitHub Actions builds, `wrangler pages deploy` uploads, on push to `main` (DEVOPS-2) |
| Safe-Mode Console (WASM) | same site, under `/safe/` | built by the same workflow, must share the web app's origin |
| API ("Ground Station") | Oracle VM, HTTPS | `ahc-api` binary + systemd + reverse proxy (DEVOPS-3) |
| Mobile APK | built locally by MOBILE | attached to the release (DEVOPS-6) |

## Rules

- **No secrets in the repo.** Passwords, tokens and the VM address go in Actions secrets or the VM's environment. `.env*` is gitignored; commit only `.env.example` with fake values.
- The astronaut side must keep working when the API is down. Never make the web build or the page load depend on the API being reachable.
- The web app gets the API URL as a build-time variable, never hard-coded.
- Basic auth on the ground routes, even though the data is fake.
- Both links must stay up until winners are announced (January 2027). Write down how to redeploy from scratch here, so anyone on the team can do it.
- Other owners ask for CI or deploy changes through a `devops:` issue with the command that works locally. Answer quickly: a red or missing check blocks their PR.
- Workflow changes follow the same PR rules as everything else, and say in the PR whether the workflow has actually run.

## Files

| File | What |
|---|---|
| `.github/workflows/deploy.yml` | push to `main`: build web (+ `/safe/` once `apps/egui/index.html` exists), `wrangler pages deploy`. Skips itself while `apps/web/package.json` is missing |
| `deploy/ahc-api.service` | systemd unit, restarts on failure, reads `/etc/ahc/api.env` |
| `deploy/Caddyfile` | HTTPS in front of the API on `127.0.0.1:8080` |
| `deploy/api.env.example` | env var names the API reads (ENGINE must match) |
| `deploy/deploy-api.sh` | `./deploy/deploy-api.sh user@host`: pull, build on the VM, restart |

## Secrets and settings

| Name | Kind | Where |
|---|---|---|
| `CLOUDFLARE_API_TOKEN` | Actions secret | token limited to Pages edit |
| `CLOUDFLARE_ACCOUNT_ID` | Actions secret | Cloudflare dashboard |
| `AHC_API_URL` | Actions **variable** | public API URL, becomes `VITE_API_URL` at build |
| `/etc/ahc/api.env` | file on the VM | ground user and password, CORS origin |

## First-time VM setup (one time, by hand)

1. `sudo useradd -r ahc; sudo mkdir -p /opt/ahc /var/lib/ahc /etc/ahc; sudo chown ahc /opt/ahc /var/lib/ahc`
2. Install Rust (rustup) and Caddy. `git clone` the repo into `/opt/ahc`.
3. Copy `deploy/api.env.example` to `/etc/ahc/api.env`, set the real password, `chmod 600`.
4. Copy `deploy/ahc-api.service` to `/etc/systemd/system/`, `sudo systemctl enable --now ahc-api`.
5. Put the real hostname in `deploy/Caddyfile`, install it as `/etc/caddy/Caddyfile`, `sudo systemctl reload caddy`.
6. Open ports 80 and 443 in the Oracle security list and the VM firewall.

## To fill in as it is built

- Public web URL (step 4a): https://ahc-37u.pages.dev (Pages project `ahc`, account Tareksheren@gmail.com)
- API VM (step 4c): `nutq-vm` (ssh alias), Oracle aarch64, 2 CPUs, shared with another project whose Docker Caddy owns 80/443. Rust and Caddy installed; port 8445 open in the security list and iptables
- Public API URL and hostname (step 4d): https://ahc12.duckdns.org:8445 (DuckDNS -> 84.8.255.31). Certificate via DuckDNS DNS challenge; token lives in /etc/ahc/duckdns.env on the VM, never in the repo
- How to redeploy the API: `./deploy/deploy-api.sh user@host`
- How to check both are up: open the web URL in a private window; `curl https://ahc12.duckdns.org:8445/api/health` returns `{"ok":true}`.
