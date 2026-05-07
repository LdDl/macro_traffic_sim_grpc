## Bump version

Update version in the following files (replace `X.Y.Z` with the new version):

1. **Rust crate:** `Cargo.toml` -> `version = "X.Y.Z"`
2. **Python client:** `clients/python/pyproject.toml` -> `version = "X.Y.Z"`
3. **Go client:** versioned via git tag (see [Git tags](#git-tags--golang-client-tag-for-pkggodev) section below)

## Regenerate clients (if proto files changed)
```bash
# Python
./scripts/gen_python.sh

# Go
./scripts/gen_go.sh
```

## Run tests
```bash
cargo test --features server
```


## Build release binaries
```bash
./scripts/build_release.sh
```

## Publish to crates.io
```bash
cargo publish
```

## Publish python client to PyPI
```bash
cd clients/python

# Activate venv
source .venv/bin/activate

# Install build tools
pip install build twine

# Build package
python -m build

# Upload to PyPI
twine upload dist/*

cd ../..
```

## Build and push Docker images
```bash
# Build
docker build -f Dockerfile.server -t macro-traffic-sim/server:latest .

# Tag for Docker Hub
docker tag macro-traffic-sim/server:latest dimahkiin/macro-traffic-sim-server:X.Y.Z
docker tag macro-traffic-sim/server:latest dimahkiin/macro-traffic-sim-server:latest

# Tag for GitHub Container Registry
docker tag macro-traffic-sim/server:latest ghcr.io/lddl/macro-traffic-sim-server:X.Y.Z
docker tag macro-traffic-sim/server:latest ghcr.io/lddl/macro-traffic-sim-server:latest

# Push to Docker Hub
docker push dimahkiin/macro-traffic-sim-server:X.Y.Z
docker push dimahkiin/macro-traffic-sim-server:latest

# Push to GitHub Container Registry
docker push ghcr.io/lddl/macro-traffic-sim-server:X.Y.Z
docker push ghcr.io/lddl/macro-traffic-sim-server:latest
```

## Git tags (+ golang client tag for pkg.go.dev)
```bash
# Main repo tag
git tag vX.Y.Z
git push origin vX.Y.Z

# Go submodule tag (for pkg.go.dev)
git tag clients/go/vX.Y.Z
git push origin clients/go/vX.Y.Z
```

## Verify releases

- **crates.io:** https://crates.io/crates/macro_traffic_sim
- **PyPI:** https://pypi.org/project/macro-traffic-sim/
- **pkg.go.dev:** https://pkg.go.dev/github.com/LdDl/macro_traffic_sim_grpc/clients/go
- **Docker Hub:** https://hub.docker.com/r/dimahkiin/macro-traffic-sim-server/tags
- **GitHub Releases:** https://github.com/LdDl/macro_traffic_sim_grpc/releases
