# MCP-Guard Installation Scripts

Automated installation scripts for Pro and Enterprise tiers.

## Overview

These scripts provide one-command installation for MCP-Guard Pro and Enterprise:

- **`install-pro.sh`**: Installs MCP-Guard Pro tier
- **`install-enterprise.sh`**: Installs MCP-Guard Enterprise tier

Both scripts:
- Auto-detect platform (Linux/macOS, Intel/ARM, glibc/musl)
- Validate license keys
- Download binaries from Cloudflare Worker
- Install to `/usr/local/bin`
- Save license configuration
- Provide helpful next steps

## Usage

### Pro Tier

```bash
# Interactive (prompts for license)
curl -fsSL https://mcp-guard.io/install-pro.sh | bash

# Non-interactive (license from environment)
curl -fsSL https://mcp-guard.io/install-pro.sh | \
  MCP_GUARD_LICENSE_KEY=pro_xxx bash

# Local testing
./scripts/install-pro.sh
```

### Enterprise Tier

```bash
# Interactive (prompts for license)
curl -fsSL https://mcp-guard.io/install-enterprise.sh | bash

# Non-interactive (license from environment)
curl -fsSL https://mcp-guard.io/install-enterprise.sh | \
  MCP_GUARD_LICENSE_KEY=ent_xxx bash

# Local testing
./scripts/install-enterprise.sh
```

## Hosting

These scripts should be hosted at:
- `https://mcp-guard.io/install-pro.sh`
- `https://mcp-guard.io/install-enterprise.sh`

### Option 1: Cloudflare Pages

```bash
# Create pages/ directory
mkdir -p pages/public

# Copy scripts
cp scripts/install-pro.sh pages/public/install-pro.sh
cp scripts/install-enterprise.sh pages/public/install-enterprise.sh

# Deploy to Cloudflare Pages
cd pages
wrangler pages deploy public
```

### Option 2: GitHub Pages

```bash
# Create gh-pages branch
git checkout -b gh-pages
git checkout main -- scripts/

# Move scripts to root
mv scripts/install-pro.sh .
mv scripts/install-enterprise.sh .

# Commit and push
git add install-*.sh
git commit -m "Add installation scripts"
git push origin gh-pages

# Access at:
# https://yourusername.github.io/mcp-guard/install-pro.sh
```

### Option 3: S3/R2 + CloudFront

```bash
# Upload to S3/R2
aws s3 cp scripts/install-pro.sh s3://mcp-guard-assets/install-pro.sh \
  --content-type "text/x-shellscript" \
  --acl public-read

# Access via CloudFront:
# https://assets.mcp-guard.io/install-pro.sh
```

## Platform Detection

Scripts automatically detect:

### Linux
- **glibc** (default): `x86_64-linux`
- **musl** (Alpine): `x86_64-linux-musl`
- Detection: Checks `ldd /bin/ls | grep musl`

### macOS
- **Intel**: `x86_64-darwin`
- **Apple Silicon**: `aarch64-darwin`
- Detection: Uses `uname -m` (x86_64 vs arm64)

### Unsupported
- Windows: Directs users to manual download page
- Other architectures: Shows error with supported platforms

## License Validation

### Pro Tier (`install-pro.sh`)
- Accepts license keys starting with `pro_`
- Validates via Cloudflare Worker (offline Ed25519 signature check)
- Saves to `~/.config/mcp-guard/.env`

### Enterprise Tier (`install-enterprise.sh`)
- Accepts license keys starting with `ent_`
- Validates via Keygen.sh API (online check)
- Checks network connectivity to `api.keygen.sh` before download
- Warns if network unreachable (Enterprise requires online validation)
- Saves to `~/.config/mcp-guard/.env`

## Error Handling

Scripts handle common errors gracefully:

1. **Unsupported platform**: Shows supported platforms and exits
2. **Missing license**: Prompts user interactively
3. **Invalid license format**: Validates `pro_` or `ent_` prefix
4. **Download failure**: Shows helpful error from Worker (expired, invalid, etc.)
5. **Permission denied**: Uses `sudo` if needed for `/usr/local/bin`
6. **Network issues**: Enterprise script checks connectivity first

## Testing

### Test Platform Detection

```bash
# Run script without license to see platform detection
./scripts/install-pro.sh
# Should show detected platform, then prompt for license
# Press Ctrl+C to cancel

# Check detection logic
bash -c 'source scripts/install-pro.sh && detect_platform'
```

### Test License Validation (Dry Run)

```bash
# Set fake license
export MCP_GUARD_LICENSE_KEY="pro_test"

# Run script (will fail at download, but tests detection + validation)
./scripts/install-pro.sh
# Should show: "Invalid license key" error from Worker
```

### Test with Real License

```bash
# Generate test Pro license
cargo run --bin sign-license -- \
  --tier pro \
  --licensee "test@example.com" \
  --expires-at "2026-12-31T23:59:59Z"

# Copy license key output
export MCP_GUARD_LICENSE_KEY="pro_xxx..."

# Run installer
./scripts/install-pro.sh
# Should download and install successfully
```

## Customization

### Change Install Directory

Edit `INSTALL_DIR` variable in scripts:

```bash
# System-wide (default)
INSTALL_DIR="/usr/local/bin"

# User-local
INSTALL_DIR="$HOME/.local/bin"

# Custom
INSTALL_DIR="/opt/mcp-guard/bin"
```

**Note**: User must ensure install directory is in `$PATH`.

### Custom Download URL

Edit `DOWNLOAD_URL` variable:

```bash
# Production (default)
DOWNLOAD_URL="https://download.mcp-guard.io/download"

# Staging
DOWNLOAD_URL="https://download-staging.mcp-guard.io/download"

# Local testing
DOWNLOAD_URL="http://localhost:8787/download"
```

## Integration Examples

### Docker

```dockerfile
FROM ubuntu:22.04

# Install MCP-Guard Pro
ARG MCP_GUARD_LICENSE_KEY
RUN curl -fsSL https://mcp-guard.io/install-pro.sh | \
    MCP_GUARD_LICENSE_KEY=${MCP_GUARD_LICENSE_KEY} bash

# Copy config
COPY mcp-guard.toml /etc/mcp-guard/mcp-guard.toml

# Run
CMD ["mcp-guard", "run", "--config", "/etc/mcp-guard/mcp-guard.toml"]
```

### Kubernetes Init Container

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: mcp-server
spec:
  initContainers:
  - name: install-mcp-guard
    image: ubuntu:22.04
    env:
    - name: MCP_GUARD_LICENSE_KEY
      valueFrom:
        secretKeyRef:
          name: mcp-guard-license
          key: license-key
    command:
    - /bin/bash
    - -c
    - |
      curl -fsSL https://mcp-guard.io/install-pro.sh | bash
      cp /usr/local/bin/mcp-guard /shared/mcp-guard
    volumeMounts:
    - name: shared
      mountPath: /shared
  containers:
  - name: mcp-guard
    image: ubuntu:22.04
    command: ["/shared/mcp-guard", "run"]
    volumeMounts:
    - name: shared
      mountPath: /shared
  volumes:
  - name: shared
    emptyDir: {}
```

### Ansible

```yaml
- name: Install MCP-Guard Pro
  hosts: mcp_servers
  tasks:
    - name: Download installer
      get_url:
        url: https://mcp-guard.io/install-pro.sh
        dest: /tmp/install-pro.sh
        mode: '0755'

    - name: Run installer
      shell: /tmp/install-pro.sh
      environment:
        MCP_GUARD_LICENSE_KEY: "{{ mcp_guard_license }}"
```

### Terraform

```hcl
resource "null_resource" "install_mcp_guard" {
  provisioner "remote-exec" {
    inline = [
      "curl -fsSL https://mcp-guard.io/install-pro.sh | MCP_GUARD_LICENSE_KEY=${var.mcp_guard_license} bash"
    ]
  }
}
```

## Security Considerations

1. **HTTPS Only**: Always use HTTPS URLs for script downloads
   - ✅ `curl -fsSL https://mcp-guard.io/install-pro.sh`
   - ❌ `curl -fsSL http://mcp-guard.io/install-pro.sh`

2. **License Protection**: License keys are secrets
   - Save to `~/.env` with `chmod 600`
   - Don't log license keys in CI/CD
   - Rotate if compromised

3. **Script Integrity**: Consider checksums for extra security
   ```bash
   # Download script
   curl -fsSL https://mcp-guard.io/install-pro.sh -o install.sh

   # Download checksum
   curl -fsSL https://mcp-guard.io/install-pro.sh.sha256 -o install.sh.sha256

   # Verify
   sha256sum -c install.sh.sha256

   # Run
   bash install.sh
   ```

4. **Principle of Least Privilege**: Scripts use `sudo` only when needed
   - If user owns `/usr/local/bin`, no sudo required
   - Otherwise, prompts for elevated privileges

## Troubleshooting

### "curl: command not found"

Install curl or wget:
```bash
# Debian/Ubuntu
sudo apt-get install curl

# RHEL/CentOS
sudo yum install curl

# macOS
brew install curl
```

### "Permission denied" when installing

Run with sudo or choose user-local install directory:
```bash
# Option 1: Run entire script with sudo
curl -fsSL https://mcp-guard.io/install-pro.sh | sudo bash

# Option 2: Edit script to use ~/.local/bin
INSTALL_DIR="$HOME/.local/bin"
```

### "Download failed" errors

Check license key:
```bash
# Verify license format
echo $MCP_GUARD_LICENSE_KEY
# Should start with "pro_" or "ent_"

# Test manually
curl "https://download.mcp-guard.io/download?tier=pro&platform=x86_64-linux&license=$MCP_GUARD_LICENSE_KEY"
```

### Platform detection fails

Specify platform manually by editing script:
```bash
# Force platform
platform="x86_64-linux"  # Instead of detect_platform
```

## Maintenance

### Update Scripts

When updating installation scripts:

1. Test locally with all platforms (use Docker for cross-platform)
2. Update version/date in script comments
3. Commit changes
4. Re-deploy to hosting (Pages/S3/CDN)
5. Test deployed version
6. Update documentation

### Versioning

Consider versioned URLs for backwards compatibility:
- `https://mcp-guard.io/install-pro.sh` (latest)
- `https://mcp-guard.io/v1/install-pro.sh` (v1.x)
- `https://mcp-guard.io/v2/install-pro.sh` (v2.x)

This allows users to pin to specific installer versions.

## Support

For issues with installation scripts:
- **GitHub**: https://github.com/yourusername/mcp-guard/issues
- **Email**: support@mcp-guard.io
- **Docs**: https://mcp-guard.io/docs/installation
