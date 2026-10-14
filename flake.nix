{
  description = "Product-neutral artifact authentication, transfer geometry, and binding mechanics";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    cairn = {
      url = "github:OnixResearch/cairn-public/acb4c3d2cb90b231e07a6b71115c79adde46349c";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      cairn,
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      eachSystem = function: nixpkgs.lib.genAttrs systems (system: function system);
    in
    {
      packages = eachSystem (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          rustPlatform = pkgs.makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
          };
        in
        {
          artifact = rustPlatform.buildRustPackage {
            pname = "artifact";
            version = "0.1.0";
            src = self;
            cargoLock.lockFile = ./Cargo.lock;
            cargoBuildFlags = [ "--workspace" ];
            cargoTestFlags = [ "--workspace" ];
            installPhase = ''
              mkdir -p "$out/share/artifact" "$out/share/doc/artifact" "$out/share/licenses/artifact"
              cp config/repository.json "$out/share/artifact/repository.json"
              cp release/profile.json "$out/share/artifact/release-profile.json"
              cp README.md "$out/share/doc/artifact/"
              cp -R docs "$out/share/doc/artifact/"
              cp LICENSE LICENSE-MIT LICENSE-APACHE NOTICE "$out/share/licenses/artifact/"
            '';
          };
          default = self.packages.${system}.artifact;
        }
      );

      checks = eachSystem (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          rustPlatform = pkgs.makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
          };
          noStdCheck =
            name: package: target:
            rustPlatform.buildRustPackage {
              pname = name;
              version = "0.1.0";
              src = self;
              cargoLock.lockFile = ./Cargo.lock;
              cargoBuildFlags = [
                "-p"
                package
                "--no-default-features"
              ]
              ++ pkgs.lib.optionals (target != null) [
                "--target"
                target
              ];
              doCheck = false;
              installPhase = ''
                touch "$out"
              '';
            };
        in
        {
          cargo-test = rustPlatform.buildRustPackage {
            pname = "artifact-workspace-tests";
            version = "0.1.0";
            src = self;
            cargoLock.lockFile = ./Cargo.lock;
            cargoBuildFlags = [ "--workspace" ];
            cargoTestFlags = [ "--workspace" ];
          };

          cargo-clippy = rustPlatform.buildRustPackage {
            pname = "artifact-workspace-clippy";
            version = "0.1.0";
            src = self;
            cargoLock.lockFile = ./Cargo.lock;
            buildPhase = ''
              cargo clippy --workspace --all-targets --offline -- -D warnings
            '';
            doCheck = false;
            installPhase = ''
              touch "$out"
            '';
          };

          cargo-format =
            pkgs.runCommand "artifact-workspace-format"
              {
                nativeBuildInputs = [ toolchain ];
                src = self;
              }
              ''
                set -eu
                cd "$src"
                cargo fmt --all -- --check
                touch "$out"
              '';

          core-no-std-host = noStdCheck "artifact-auth-core-no-std-host" "artifact-auth-core" null;
          core-no-std-wasm =
            noStdCheck "artifact-auth-core-no-std-wasm" "artifact-auth-core"
              "wasm32-unknown-unknown";
          ed25519-no-std-host = noStdCheck "artifact-auth-ed25519-no-std-host" "artifact-auth-ed25519" null;
          ed25519-no-std-wasm =
            noStdCheck "artifact-auth-ed25519-no-std-wasm" "artifact-auth-ed25519"
              "wasm32-unknown-unknown";
          transfer-no-std-host =
            noStdCheck "artifact-transfer-core-no-std-host" "artifact-transfer-core"
              null;
          transfer-no-std-wasm =
            noStdCheck "artifact-transfer-core-no-std-wasm" "artifact-transfer-core"
              "wasm32-unknown-unknown";
          binding-no-std-host = noStdCheck "artifact-binding-core-no-std-host" "artifact-binding-core" null;
          binding-no-std-wasm =
            noStdCheck "artifact-binding-core-no-std-wasm" "artifact-binding-core"
              "wasm32-unknown-unknown";
          binding-no-std-thumb =
            noStdCheck "artifact-binding-core-no-std-thumb" "artifact-binding-core"
              "thumbv7em-none-eabi";

          # r[verify artifact_binding.binding.transition]
          # r[verify artifact_binding.binding.resolution]
          # r[verify artifact_binding.binding.snapshot_consistency]
          # r[verify artifact_binding.reachability.trace]
          # r[verify artifact_binding.retirement.classification]
          # r[verify artifact_binding.diagnostics]
          publication-incubation = rustPlatform.buildRustPackage {
            pname = "durable-file-publication-incubation";
            version = "0.1.0";
            src = self + /fixtures/publication-candidate;
            cargoLock.lockFile = ./fixtures/publication-candidate/Cargo.lock;
            postCheck = ''
              cargo clippy --all-targets --offline -- --deny warnings
              cargo check --no-default-features --target wasm32-unknown-unknown --offline
            '';
            installPhase = ''
              touch "$out"
            '';
          };

          binding-core = rustPlatform.buildRustPackage {
            pname = "artifact-binding-core-check";
            version = "0.1.0";
            src = self;
            cargoLock.lockFile = ./Cargo.lock;
            cargoBuildFlags = [
              "-p"
              "artifact-binding-core"
              "--no-default-features"
            ];
            cargoTestFlags = [
              "-p"
              "artifact-binding-core"
            ];
            postCheck = ''
              cargo clippy -p artifact-binding-core --all-targets --offline -- --deny warnings
              cargo check -p artifact-binding-core --no-default-features --target wasm32-unknown-unknown --offline
              cargo check -p artifact-binding-core --no-default-features --target thumbv7em-none-eabi --offline
            '';
            installPhase = ''
              touch "$out"
            '';
          };

          nickel-contracts =
            pkgs.runCommand "artifact-workspace-nickel-contracts"
              {
                nativeBuildInputs = [ pkgs.nickel ];
                src = self;
              }
              ''
                set -eu
                cd "$src"
                nickel typecheck config/repository.ncl
                nickel typecheck config/consumers/cairn.ncl
                nickel typecheck config/consumers/mantle.ncl
                nickel typecheck config/consumers/molten.ncl
                nickel typecheck config/consumers/valence.ncl
                nickel typecheck release/profile.ncl
                nickel export --format json config/repository.ncl > "$TMPDIR/repository.json"
                nickel export --format json config/consumers/cairn.ncl > "$TMPDIR/cairn.json"
                nickel export --format json config/consumers/mantle.ncl > "$TMPDIR/mantle.json"
                nickel export --format json config/consumers/molten.ncl > "$TMPDIR/molten.json"
                nickel export --format json config/consumers/valence.ncl > "$TMPDIR/valence.json"
                nickel export --format json release/profile.ncl > "$TMPDIR/profile.json"
                cmp "$TMPDIR/repository.json" config/repository.json
                cmp "$TMPDIR/cairn.json" fixtures/consumers/cairn.json
                cmp "$TMPDIR/mantle.json" fixtures/consumers/mantle.json
                cmp "$TMPDIR/molten.json" fixtures/consumers/molten.json
                cmp "$TMPDIR/valence.json" fixtures/consumers/valence.json
                cmp "$TMPDIR/profile.json" release/profile.json
                if nickel export --format json config/fixtures/invalid-license.ncl > /dev/null 2>&1; then
                  echo "invalid standalone license unexpectedly passed" >&2
                  exit 1
                fi
                if nickel export --format json config/fixtures/missing-owner-authorization.ncl > /dev/null 2>&1; then
                  echo "missing owner authorization unexpectedly passed" >&2
                  exit 1
                fi
                if nickel export --format json config/fixtures/false-owner-authorization.ncl > /dev/null 2>&1; then
                  echo "false owner authorization unexpectedly passed" >&2
                  exit 1
                fi
                if nickel export --format json config/fixtures/mismatched-source-provenance.ncl > /dev/null 2>&1; then
                  echo "mismatched source provenance unexpectedly passed" >&2
                  exit 1
                fi
                if nickel export --format json config/fixtures/weakened-non-claims.ncl > /dev/null 2>&1; then
                  echo "weakened non-claims unexpectedly passed" >&2
                  exit 1
                fi
                if nickel export --format json config/fixtures/mapping-drops-retained-authority.ncl > /dev/null 2>&1; then
                  echo "consumer mapping authority loss unexpectedly passed" >&2
                  exit 1
                fi
                if nickel export --format json config/fixtures/forbidden-workspace-dependency.ncl > /dev/null 2>&1; then
                  echo "forbidden cross-mechanism dependency unexpectedly passed" >&2
                  exit 1
                fi
                touch "$out"
              '';

          radicle-publication =
            pkgs.runCommand "artifact-auth-radicle-publication"
              {
                nativeBuildInputs = [
                  pkgs.b3sum
                  pkgs.jq
                  pkgs.nickel
                ];
                src = self;
              }
              ''
                set -eu
                cd "$src"
                nickel typecheck evidence/radicle/publication-v1.ncl
                nickel typecheck lib/radicle-publication-receipt.ncl
                nickel export --format json tests/radicle-publication.ncl > "$TMPDIR/tests.json"
                grep -Fq '"tests": true' "$TMPDIR/tests.json"
                nickel export --format json evidence/radicle/publication-v1.ncl > "$TMPDIR/publication-v1.json"
                jq -S . "$TMPDIR/publication-v1.json" > "$TMPDIR/publication-v1.normalized.json"
                jq -S . evidence/radicle/publication-v1.json > "$TMPDIR/evidence.normalized.json"
                cmp "$TMPDIR/publication-v1.normalized.json" "$TMPDIR/evidence.normalized.json"
                actual_hash="$(b3sum evidence/radicle/publication-v1.json | cut -d ' ' -f 1)"
                expected_hash="$(tr -d '\n' < evidence/radicle/publication-v1.blake3)"
                test "$actual_hash" = "$expected_hash"
                touch "$out"
              '';

          # r[verify artifact_auth.core.authority_free]
          # r[verify artifact_auth.conformance.consumers]
          cairn-validation =
            pkgs.runCommand "artifact-workspace-cairn-validation"
              {
                nativeBuildInputs = [
                  cairn.packages.${system}.cairn
                  pkgs.nickel
                ];
                src = self;
              }
              ''
                set -eu
                cp -R --no-preserve=mode "$src" work
                cd work
                nickel export --format json cairn-policy/default.ncl \
                  > cairn-policy/generated/cairn-policy.check.json
                cmp cairn-policy/generated/cairn-policy.check.json cairn-policy/generated/cairn-policy.json
                rm cairn-policy/generated/cairn-policy.check.json
                cairn validate --root . --policy cairn-policy/generated/cairn-policy.json
                cairn traceability coverage \
                  --root . \
                  --policy cairn-policy/generated/cairn-policy.json \
                  --json > "$TMPDIR/traceability.json"
                touch "$out"
              '';

          # r[verify artifact_auth.core.authority_free]
          authority-audit =
            pkgs.runCommand "artifact-workspace-authority-audit"
              {
                nativeBuildInputs = [
                  pkgs.gnugrep
                  pkgs.jq
                  toolchain
                ];
                src = self;
              }
              ''
                set -eu
                if grep -R -E 'std::fs|std::env|std::process|std::time|std::net|tokio::|async |println!|eprintln!|pub fn sign|pub struct SigningKey|pub struct SecretKey|rand::' "$src"/crates/*/src --include='*.rs'; then
                  echo "forbidden authority or private-key dependency found" >&2
                  exit 1
                fi
                if grep -R -i -E 'cairn|mantle|crunch|aspen|molten|valence' "$src"/Cargo.toml "$src"/crates/*/Cargo.toml; then
                  echo "product dependency found in package manifests" >&2
                  exit 1
                fi
                export CARGO_HOME="$TMPDIR/cargo-home"
                cargo metadata \
                  --manifest-path "$src/Cargo.toml" \
                  --format-version 1 \
                  --no-deps \
                  --offline > "$TMPDIR/metadata.json"
                expected_package_count=4
                observed_license_count="$(grep -oF '"license":"MIT OR Apache-2.0"' "$TMPDIR/metadata.json" | wc -l)"
                if [ "$observed_license_count" -ne "$expected_package_count" ]; then
                  echo "package metadata does not carry the workspace license grant" >&2
                  exit 1
                fi
                expected_no_internal_dependencies='[]'
                expected_verifier_internal_dependencies='["artifact-auth-core"]'
                auth_core_internal_dependencies="$(jq -c '[.packages[] | select(.name == "artifact-auth-core") | .dependencies[] | select(.name | startswith("artifact-")) | .name] | sort' "$TMPDIR/metadata.json")"
                verifier_internal_dependencies="$(jq -c '[.packages[] | select(.name == "artifact-auth-ed25519") | .dependencies[] | select(.name | startswith("artifact-")) | .name] | sort' "$TMPDIR/metadata.json")"
                transfer_internal_dependencies="$(jq -c '[.packages[] | select(.name == "artifact-transfer-core") | .dependencies[] | select(.name | startswith("artifact-")) | .name] | sort' "$TMPDIR/metadata.json")"
                binding_internal_dependencies="$(jq -c '[.packages[] | select(.name == "artifact-binding-core") | .dependencies[] | select(.name | startswith("artifact-")) | .name] | sort' "$TMPDIR/metadata.json")"
                if [ "$auth_core_internal_dependencies" != "$expected_no_internal_dependencies" ]; then
                  echo "artifact-auth-core gained a forbidden workspace dependency" >&2
                  exit 1
                fi
                if [ "$verifier_internal_dependencies" != "$expected_verifier_internal_dependencies" ]; then
                  echo "artifact-auth-ed25519 workspace dependency allowlist changed" >&2
                  exit 1
                fi
                if [ "$transfer_internal_dependencies" != "$expected_no_internal_dependencies" ]; then
                  echo "artifact-transfer-core gained a forbidden workspace dependency" >&2
                  exit 1
                fi
                if [ "$binding_internal_dependencies" != "$expected_no_internal_dependencies" ]; then
                  echo "artifact-binding-core gained a forbidden workspace dependency" >&2
                  exit 1
                fi
                grep -F 'explicitly authorized reuse, modification, and relicensing' "$src/provenance/SOURCE_GRANT.md" > /dev/null
                grep -F '26c6880c08cd730c9e9420f92ea91a87212e492d' "$src/provenance/SOURCE_GRANT.md" > /dev/null
                grep -F 'cc4639be24475d80844b091cdec605eb2fef9ff0' "$src/provenance/SOURCE_GRANT.md" > /dev/null
                grep -F '70590459a218dc8e66948ab6f305a7c54142b710' "$src/provenance/SOURCE_GRANT.md" > /dev/null
                grep -F '8eccac35fcd8aae8ef64b3d1021e7f1a53656180' "$src/provenance/SOURCE_GRANT.md" > /dev/null
                grep -F '507b10823c4513064710a70fe565f9a1ee592985' "$src/provenance/SOURCE_GRANT.md" > /dev/null
                touch "$out"
              '';
        }
      );

      devShells = eachSystem (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        in
        {
          default = pkgs.mkShell {
            packages = [
              toolchain
              pkgs.nickel
              pkgs.nixfmt
            ];
          };
        }
      );
    };
}
