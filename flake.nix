{
  description = "CIM Subject - Subject Algebra for NATS-based domain routing";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, crane, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        src = craneLib.cleanCargoSource ./.;

        commonArgs = {
          inherit src;
          strictDeps = true;

          buildInputs = with pkgs; [
            openssl
            pkg-config
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            libiconv
            darwin.apple_sdk.frameworks.Security
            darwin.apple_sdk.frameworks.SystemConfiguration
          ];

          nativeBuildInputs = with pkgs; [
            pkg-config
            rustToolchain
          ];
        };

        # Build the crate as part of `nix build`
        cim-subject = craneLib.buildPackage (commonArgs // {
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          
          # Run tests as part of the build
          doCheck = true;
        });

        # Development shell with all tools
        devShell = craneLib.devShell {
          # Inherit inputs from commonArgs
          inherit (commonArgs) buildInputs nativeBuildInputs;

          # Additional tools for development
          packages = with pkgs; [
            # Rust tools
            cargo-watch
            cargo-edit
            cargo-outdated
            cargo-audit
            cargo-tarpaulin
            cargo-nextest
            cargo-machete
            cargo-deny
            rust-analyzer
            clippy
            rustfmt

            # NATS tools
            natscli
            nats-server

            # Development tools
            git
            gh
            jq
            ripgrep
            fd
            bat
            eza
            tokei
            hyperfine
            just

            # Documentation tools
            mdbook
            graphviz
            plantuml
          ];

          # Environment variables
          RUST_LOG = "debug";
          RUST_BACKTRACE = "1";
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";

          # Shell hook
          shellHook = ''
            echo "🦀 Welcome to CIM Subject development environment!"
            echo ""
            echo "Available commands:"
            echo "  cargo build          - Build the project"
            echo "  cargo test           - Run tests"
            echo "  cargo run --example  - Run examples"
            echo "  cargo doc --open     - Build and open documentation"
            echo "  cargo watch -x test  - Watch and run tests"
            echo "  nats-server -js      - Start NATS with JetStream"
            echo ""
            echo "Project version: $(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)"
            echo "Rust version: $(rustc --version)"
            echo ""
          '';
        };

      in
      {
        # Package output
        packages = {
          default = cim-subject;
          cim-subject = cim-subject;
        };

        # Development shell
        devShells.default = devShell;

        # Apps for running examples
        apps = {
          # Basic examples
          basic-routing = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "basic-routing" ''
              ${cim-subject}/bin/basic_routing
            '';
          };

          correlation-tracking = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "correlation-tracking" ''
              ${cim-subject}/bin/correlation_tracking
            '';
          };

          # Mortgage lending examples
          mortgage-routing = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "mortgage-routing" ''
              ${cim-subject}/bin/mortgage_lending_routing
            '';
          };

          document-validation = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "document-validation" ''
              ${cim-subject}/bin/document_validation
            '';
          };

          rate-shopping = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "rate-shopping" ''
              ${cim-subject}/bin/rate_shopping
            '';
          };
        };

        # Checks run by `nix flake check`
        checks = {
          inherit cim-subject;

          # Format check
          fmt = craneLib.cargoFmt {
            inherit src;
          };

          # Clippy check
          clippy = craneLib.cargoClippy (commonArgs // {
            cargoArtifacts = craneLib.buildDepsOnly commonArgs;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          # Documentation check
          doc = craneLib.cargoDoc (commonArgs // {
            cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          });

          # Audit check
          audit = craneLib.cargoAudit {
            inherit src;
            inherit (pkgs) advisory-db;
          };

          # Deny check
          deny = craneLib.cargoDeny {
            inherit src;
          };
        };

        # Container configuration
        nixosConfigurations.container = nixpkgs.lib.nixosSystem {
          inherit system;
          modules = [
            self.nixosModules.default
            ({ config, pkgs, ... }: {
              boot.isContainer = true;
              networking.useDHCP = false;
              
              services.cim-subject = {
                enable = true;
                natsUrl = "10.233.1.1:4222";  # Adjust for your network
                logLevel = "info";
              };
              
              # Container-specific settings
              system.stateVersion = "23.11";
              networking.firewall.enable = false;
              
              # Minimal container
              documentation.enable = false;
              programs.command-not-found.enable = false;
            })
          ];
        };

        # Module for NixOS/Home Manager integration
        nixosModules.default = { config, lib, pkgs, ... }:
          with lib;
          let
            cfg = config.services.cim-subject;
          in
          {
            options.services.cim-subject = {
              enable = mkEnableOption "CIM Subject service";

              package = mkOption {
                type = types.package;
                default = self.packages.${system}.cim-subject;
                description = "The CIM Subject package to use";
              };

              natsUrl = mkOption {
                type = types.str;
                default = "localhost:4222";
                description = "NATS server URL";
              };

              logLevel = mkOption {
                type = types.enum [ "trace" "debug" "info" "warn" "error" ];
                default = "info";
                description = "Log level for the service";
              };
            };

            config = mkIf cfg.enable {
              environment.systemPackages = [ cfg.package ];

              systemd.services.cim-subject = {
                description = "CIM Subject Service";
                after = [ "network.target" "nats.service" ];
                wants = [ "nats.service" ];

                environment = {
                  RUST_LOG = cfg.logLevel;
                  NATS_URL = cfg.natsUrl;
                };

                serviceConfig = {
                  Type = "notify";
                  ExecStart = "${cfg.package}/bin/cim-subject-service";
                  Restart = "on-failure";
                  RestartSec = "5s";

                  # Security hardening
                  DynamicUser = true;
                  NoNewPrivileges = true;
                  ProtectSystem = "strict";
                  ProtectHome = true;
                  PrivateTmp = true;
                  PrivateDevices = true;
                  ProtectKernelTunables = true;
                  ProtectKernelModules = true;
                  ProtectControlGroups = true;
                  RestrictAddressFamilies = [ "AF_INET" "AF_INET6" "AF_UNIX" ];
                  RestrictNamespaces = true;
                  LockPersonality = true;
                  RestrictRealtime = true;
                  RestrictSUIDSGID = true;
                  SystemCallFilter = [ "@system-service" "~@privileged" ];
                };
              };
            };
          };
      });
}