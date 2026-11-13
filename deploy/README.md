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

## To fill in as it is built

- Public web URL (step 4a):
- Public API URL and hostname (step 4d):
- Where the secrets live and what each is called:
- How to redeploy the API:
- How to check both are up:
