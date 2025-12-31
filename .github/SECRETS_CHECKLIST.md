# GitHub Secrets Checklist

Quick reference for configuring repository secrets.

## Required Secrets

Go to: **Settings → Secrets and variables → Actions → New repository secret**

### 1. Cloudflare R2 (For Pro/Enterprise Binary Storage)

| Secret Name | Where to Find | Example Value |
|-------------|---------------|---------------|
| `R2_ACCESS_KEY_ID` | Cloudflare Dashboard → R2 → Manage R2 API Tokens → Create Token | `4a7b2f8c9d1e3f5g` |
| `R2_SECRET_ACCESS_KEY` | Same place (shown once) | `a1b2c3d4e5f6...` |
| `R2_ENDPOINT` | Cloudflare Dashboard → R2 → Overview → Account ID | `https://abc123.r2.cloudflarestorage.com` |

**Setup Steps:**
1. Cloudflare Dashboard → R2 → Manage R2 API Tokens
2. Create API Token
3. Permissions: Object Read & Write
4. Bucket Scope: `mcp-guard-pro-binaries`, `mcp-guard-enterprise-binaries`
5. Copy Access Key ID and Secret Access Key

### 2. Crates.io (For Publishing Free Tier)

| Secret Name | Where to Find | Example Value |
|-------------|---------------|---------------|
| `CARGO_REGISTRY_TOKEN` | https://crates.io/settings/tokens | `cio_abc123...` |

**Setup Steps:**
1. Login to https://crates.io
2. Account Settings → API Tokens → New Token
3. Name: "GitHub Actions CI"
4. Scopes: `publish-update`
5. Copy token (shown once!)

---

## Optional Secrets (For Future Phases)

### 3. Stripe (Payment Processing)

| Secret Name | Where to Find | When Needed |
|-------------|---------------|-------------|
| `STRIPE_SECRET_KEY` | Stripe Dashboard → Developers → API Keys | Phase 4: Customer portal |
| `STRIPE_WEBHOOK_SECRET` | Stripe Dashboard → Webhooks → Add endpoint | Phase 4: Customer portal |
| `STRIPE_PRO_PRICE_ID` | Stripe Dashboard → Products → MCP-Guard Pro | Phase 4: Customer portal |

### 4. Cloudflare Workers (License Signing)

| Secret Name | Where to Find | When Needed |
|-------------|---------------|-------------|
| (none - use wrangler secrets) | `wrangler secret put` | Phase 1: Vault setup |

**Note:** Cloudflare Worker secrets are managed via Wrangler CLI, not GitHub.

### 5. Email Service (License Delivery)

| Secret Name | Where to Find | When Needed |
|-------------|---------------|-------------|
| `RESEND_API_KEY` | https://resend.com/api-keys | Phase 4: Customer portal |

---

## Verification

### Test R2 Access

```bash
# Set environment variables
export AWS_ACCESS_KEY_ID="<R2_ACCESS_KEY_ID>"
export AWS_SECRET_ACCESS_KEY="<R2_SECRET_ACCESS_KEY>"

# Test list buckets
aws s3 ls --endpoint-url https://<account-id>.r2.cloudflarestorage.com

# Should list: mcp-guard-pro-binaries, mcp-guard-enterprise-binaries
```

### Test Crates.io Token

```bash
# Set token
export CARGO_REGISTRY_TOKEN="<your-token>"

# Test with dry run
cargo publish --dry-run -p mcp-guard-core

# Should succeed without errors
```

---

## Security Best Practices

✅ **DO:**
- Rotate secrets every 6-12 months
- Use repository secrets (not environment secrets)
- Delete unused tokens/keys
- Monitor secret usage in audit logs

❌ **DON'T:**
- Share secrets via email/chat
- Commit secrets to repository
- Use same token for multiple purposes
- Give secrets broader permissions than needed

---

## Quick Setup Commands

### Create All Secrets at Once (GitHub CLI)

```bash
# Install GitHub CLI: https://cli.github.com

# Set repository (replace with your repo)
REPO="botzrdev/mcp-guard"

# Add R2 secrets
gh secret set R2_ACCESS_KEY_ID --repo $REPO
gh secret set R2_SECRET_ACCESS_KEY --repo $REPO
gh secret set R2_ENDPOINT --repo $REPO

# Add crates.io secret
gh secret set CARGO_REGISTRY_TOKEN --repo $REPO

# Verify
gh secret list --repo $REPO
```

**Tip:** The `gh secret set` command will prompt you to paste the secret value.

---

## Troubleshooting

### Secret not working in workflow

1. **Check spelling:** Secret names are case-sensitive
2. **Check scope:** Must be repository secret, not environment secret
3. **Re-create:** Delete and recreate if corrupted
4. **Check logs:** Workflow logs will show "Secret not found" error

### R2 upload fails

- Verify endpoint URL format: `https://<id>.r2.cloudflarestorage.com`
- Check bucket names exactly match workflow
- Ensure token has Object Read & Write permissions
- Verify token is scoped to correct buckets

### Crates.io publish fails

- Verify token has `publish-update` scope
- Check token hasn't expired
- Ensure you're a crate owner (for updates)
- Try `cargo login` locally with same token

---

## Status

- [ ] `R2_ACCESS_KEY_ID` configured
- [ ] `R2_SECRET_ACCESS_KEY` configured
- [ ] `R2_ENDPOINT` configured
- [ ] `CARGO_REGISTRY_TOKEN` configured
- [ ] Tested R2 access with AWS CLI
- [ ] Tested crates.io token with dry-run
- [ ] Ran workflow successfully
