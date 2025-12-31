# Release Workflow Setup Guide

This guide explains how to configure GitHub Actions and Cloudflare R2 for multi-tier releases.

## Overview

We have two release workflows:

1. **`release.yml`** (Legacy) - Original single-tier release workflow
   - Builds free tier only
   - Creates GitHub releases
   - Publishes to crates.io
   - **Status:** Keep for now, will deprecate after testing new workflow

2. **`release-tiers.yml`** (New) - Multi-tier release workflow
   - Builds Free, Pro, and Enterprise tiers separately
   - Uploads Pro/Enterprise to Cloudflare R2 (gated downloads)
   - Publishes Free tier to GitHub + crates.io
   - **Status:** Use this for v1.0+ releases

---

## Prerequisites

### 1. Cloudflare Account Setup

#### A. Create R2 Buckets

1. Log in to Cloudflare Dashboard
2. Navigate to **R2 Object Storage**
3. Click **Create bucket**

Create two buckets:
- **Bucket 1:** `mcp-guard-pro-binaries` (Private)
- **Bucket 2:** `mcp-guard-enterprise-binaries` (Private)

**Important:** Keep buckets PRIVATE. Access will be gated through Cloudflare Workers.

#### B. Generate R2 API Tokens

1. In Cloudflare Dashboard → **R2** → **Manage R2 API Tokens**
2. Click **Create API Token**
3. Configure:
   - **Token name:** GitHub Actions Release
   - **Permissions:** Object Read & Write
   - **Bucket scope:** Both buckets (pro + enterprise)
   - **TTL:** No expiry (or 10 years)

4. **Copy credentials:**
   - Access Key ID (looks like: `abc123...`)
   - Secret Access Key (looks like: `xyz789...`)

5. **Get R2 endpoint:**
   - Format: `https://<ACCOUNT_ID>.r2.cloudflarestorage.com`
   - Your account ID is in Dashboard → R2 → Overview

---

### 2. GitHub Secrets Setup

Add the following secrets to your GitHub repository:

**Settings → Secrets and variables → Actions → New repository secret**

| Secret Name | Value | Description |
|-------------|-------|-------------|
| `R2_ACCESS_KEY_ID` | `<your-access-key>` | R2 API token access key |
| `R2_SECRET_ACCESS_KEY` | `<your-secret-key>` | R2 API token secret key |
| `R2_ENDPOINT` | `https://<account-id>.r2.cloudflarestorage.com` | Your R2 endpoint URL |
| `CARGO_REGISTRY_TOKEN` | `<crates-io-token>` | Token from crates.io (for publishing) |

#### How to Get `CARGO_REGISTRY_TOKEN`:

1. Log in to https://crates.io
2. Go to **Account Settings** → **API Tokens**
3. Click **New Token**
4. Name: "GitHub Actions CI"
5. Scopes: `publish-update` (or `publish-new` for first publish)
6. **Copy token** (you won't see it again!)

---

## Usage

### Automatic Release (Tag Push)

When you push a version tag, the workflow automatically builds all tiers:

```bash
# Create and push a version tag
git tag v1.0.0
git push origin v1.0.0
```

**What happens:**
1. ✅ Builds Free tier for all platforms
2. ✅ Builds Pro tier for all platforms
3. ✅ Builds Enterprise tier for all platforms
4. ✅ Creates GitHub release with Free tier binaries
5. ✅ Uploads Pro binaries to R2 bucket
6. ✅ Uploads Enterprise binaries to R2 bucket
7. ✅ Publishes mcp-guard-core to crates.io (if stable release)

### Manual Release (Workflow Dispatch)

You can manually trigger builds for specific tiers:

1. Go to **Actions** → **Release All Tiers**
2. Click **Run workflow**
3. Select tier:
   - `all` - Build all tiers
   - `free` - Build free tier only
   - `pro` - Build pro tier only
   - `enterprise` - Build enterprise tier only

**Use cases:**
- Rebuild Pro tier after fixing a bug (without releasing free tier)
- Test enterprise build before official release
- Hotfix a specific tier

---

## Verification

### Check Free Tier Release

After workflow completes:

1. **GitHub Release:**
   - Go to https://github.com/YOUR_USERNAME/mcp-guard/releases
   - Should see `mcp-guard Free vX.Y.Z`
   - Binaries: `mcp-guard-free-x86_64-linux.tar.gz`, etc.

2. **crates.io:**
   - Visit https://crates.io/crates/mcp-guard-core
   - Should show latest version

### Check Pro Tier Upload

```bash
# Install AWS CLI
brew install awscli  # or apt-get install awscli

# Configure for R2
export AWS_ACCESS_KEY_ID="<your-r2-access-key>"
export AWS_SECRET_ACCESS_KEY="<your-r2-secret-key>"
export AWS_ENDPOINT_URL="https://<account-id>.r2.cloudflarestorage.com"

# List Pro binaries
aws s3 ls s3://mcp-guard-pro-binaries/latest/ --endpoint-url $AWS_ENDPOINT_URL

# Should show:
# mcp-guard-pro-x86_64-linux.tar.gz
# mcp-guard-pro-x86_64-darwin.tar.gz
# mcp-guard-pro-aarch64-darwin.tar.gz
# mcp-guard-pro-x86_64-windows.exe.zip
```

### Check Enterprise Tier Upload

```bash
# List Enterprise binaries
aws s3 ls s3://mcp-guard-enterprise-binaries/latest/ --endpoint-url $AWS_ENDPOINT_URL

# Should show enterprise binaries
```

---

## Troubleshooting

### Error: `R2_ACCESS_KEY_ID` not found

**Problem:** GitHub secret not configured.

**Solution:**
1. Go to Settings → Secrets and variables → Actions
2. Add `R2_ACCESS_KEY_ID` secret with your R2 access key

### Error: Access Denied when uploading to R2

**Problem:** R2 API token doesn't have write permissions to bucket.

**Solution:**
1. Check R2 API token permissions include **Object Read & Write**
2. Verify token is scoped to both pro and enterprise buckets
3. Regenerate token if needed

### Error: `cargo publish` failed

**Problem:** `CARGO_REGISTRY_TOKEN` is missing or invalid.

**Solution:**
1. Get new token from https://crates.io/settings/tokens
2. Update `CARGO_REGISTRY_TOKEN` secret in GitHub

### Error: Build fails with "feature not found"

**Problem:** Feature flags not properly configured in Cargo.toml.

**Solution:**
```toml
# In Cargo.toml, ensure features are defined:
[features]
pro = []
enterprise = ["pro"]
```

### Pro/Enterprise binaries not appearing in R2

**Problem:** Workflow succeeded but files not uploaded.

**Solution:**
1. Check workflow logs for R2 upload step
2. Verify R2 endpoint URL is correct (format: `https://<id>.r2.cloudflarestorage.com`)
3. Check bucket names match exactly: `mcp-guard-pro-binaries`, `mcp-guard-enterprise-binaries`

---

## Binary Naming Convention

### Free Tier (Public GitHub Release)
```
mcp-guard-free-x86_64-linux.tar.gz
mcp-guard-free-x86_64-linux-musl.tar.gz
mcp-guard-free-x86_64-darwin.tar.gz
mcp-guard-free-aarch64-darwin.tar.gz
mcp-guard-free-x86_64-windows.exe.zip
```

### Pro Tier (Private R2)
```
mcp-guard-pro-x86_64-linux.tar.gz
mcp-guard-pro-x86_64-linux-musl.tar.gz
mcp-guard-pro-x86_64-darwin.tar.gz
mcp-guard-pro-aarch64-darwin.tar.gz
mcp-guard-pro-x86_64-windows.exe.zip
```

### Enterprise Tier (Private R2)
```
mcp-guard-enterprise-x86_64-linux.tar.gz
mcp-guard-enterprise-x86_64-linux-musl.tar.gz
mcp-guard-enterprise-x86_64-darwin.tar.gz
mcp-guard-enterprise-aarch64-darwin.tar.gz
mcp-guard-enterprise-x86_64-windows.exe.zip
```

---

## R2 Storage Structure

### Pro Binaries Bucket

```
s3://mcp-guard-pro-binaries/
├── releases/
│   ├── 1.0.0/
│   │   ├── mcp-guard-pro-x86_64-linux.tar.gz
│   │   ├── mcp-guard-pro-x86_64-darwin.tar.gz
│   │   └── ...
│   ├── 1.0.1/
│   │   └── ...
│   └── 1.1.0/
│       └── ...
└── latest/
    ├── mcp-guard-pro-x86_64-linux.tar.gz
    ├── mcp-guard-pro-x86_64-darwin.tar.gz
    └── ...
```

**Note:** Both `releases/<version>/` and `latest/` are updated on each release.

### Enterprise Binaries Bucket

Same structure as Pro, but in `s3://mcp-guard-enterprise-binaries/`

---

## Migration from Old Workflow

If you want to fully migrate to the new workflow:

1. **Test new workflow:**
   ```bash
   # Trigger manual build for free tier only
   # GitHub Actions → Release All Tiers → Run workflow → Select "free"
   ```

2. **Verify outputs match old workflow**

3. **Disable old workflow (optional):**
   ```bash
   # Rename to disable
   mv .github/workflows/release.yml .github/workflows/release.yml.disabled
   ```

4. **Or keep both:**
   - Use `release-tiers.yml` for multi-tier releases
   - Keep `release.yml` as backup

---

## Next Steps

After setting up this workflow:

1. ✅ Configure R2 buckets
2. ✅ Add GitHub secrets
3. ✅ Test with manual workflow dispatch
4. ✅ Create test tag to verify automated release
5. ✅ Build Cloudflare Worker for gated binary downloads (see `workers/download-binary/`)
6. ✅ Set up installation scripts (`scripts/install-pro.sh`)

---

## Security Considerations

### R2 Bucket Access

- ✅ **DO:** Keep R2 buckets PRIVATE
- ✅ **DO:** Use Cloudflare Worker to gate access
- ✅ **DO:** Verify license before serving binaries
- ❌ **DON'T:** Make buckets public (bypasses licensing)
- ❌ **DON'T:** Share R2 API credentials

### GitHub Secrets

- ✅ **DO:** Rotate R2 API tokens every 6-12 months
- ✅ **DO:** Use repository secrets (not environment secrets for public repos)
- ✅ **DO:** Limit token permissions to only what's needed
- ❌ **DON'T:** Commit secrets to repository
- ❌ **DON'T:** Log secrets in workflow output

---

## Support

If you encounter issues:

1. Check workflow logs in GitHub Actions tab
2. Verify all secrets are configured correctly
3. Test R2 access with AWS CLI manually
4. Review this guide for common troubleshooting steps

For Cloudflare R2 issues:
- Cloudflare Community: https://community.cloudflare.com
- R2 Documentation: https://developers.cloudflare.com/r2/

For GitHub Actions issues:
- GitHub Community: https://github.community
- Actions Documentation: https://docs.github.com/actions
