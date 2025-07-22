# Copyright (c) 2025 Cowboy AI, LLC.
#
# Multi-container composition for CIM ecosystem
#
# This file demonstrates how to deploy multiple CIM services together
# using extra-container for a complete messaging infrastructure.
#
# Usage:
#   sudo extra-container create --start < compose.nix

{ config, pkgs, lib, ... }:

{
  # NATS Server Container
  containers.nats = {
    autoStart = true;
    privateNetwork = true;
    hostAddress = "10.233.1.1";
    localAddress = "10.233.1.2";

    config = { config, pkgs, ... }: {
      # Install and configure NATS
      environment.systemPackages = [ pkgs.nats-server ];
      
      # NATS configuration
      environment.etc."nats-server.conf".text = ''
        port: 4222
        
        jetstream {
          store_dir: /var/lib/nats/jetstream
          max_memory_store: 1GB
          max_file_store: 10GB
        }
        
        cluster {
          name: CIM_CLUSTER
          port: 6222
          routes: []
        }
        
        # Authentication
        accounts {
          CIM {
            jetstream: enabled
            users: [
              { user: cim-subject, password: "$2a$11$PWIFAL8RsWyGI3jVZtO8Nu" }
            ]
          }
        }
        
        # Monitoring
        http_port: 8222
      '';

      # NATS systemd service
      systemd.services.nats = {
        description = "NATS Server";
        after = [ "network.target" ];
        wantedBy = [ "multi-user.target" ];
        
        serviceConfig = {
          ExecStart = "${pkgs.nats-server}/bin/nats-server -c /etc/nats-server.conf";
          Restart = "on-failure";
          RestartSec = "5s";
          User = "nats";
          Group = "nats";
        };
      };

      # Create nats user
      users.users.nats = {
        isSystemUser = true;
        group = "nats";
        home = "/var/lib/nats";
        createHome = true;
      };
      users.groups.nats = {};

      # Firewall
      networking.firewall = {
        enable = true;
        allowedTCPPorts = [ 4222 6222 8222 ];
      };

      # Container basics
      boot.isContainer = true;
      system.stateVersion = "23.11";
      networking.useDHCP = false;
    };
  };

  # CIM-Subject Service Container
  containers.cim-subject = {
    autoStart = true;
    privateNetwork = true;
    hostAddress = "10.233.2.1";
    localAddress = "10.233.2.2";

    config = { config, pkgs, ... }: {
      imports = [
        (builtins.getFlake "github:thecowboyai/cim-subject").nixosModules.default
      ];

      services.cim-subject = {
        enable = true;
        natsUrl = "10.233.1.2:4222";
        logLevel = "info";
      };

      # Container basics
      boot.isContainer = true;
      system.stateVersion = "23.11";
      networking.useDHCP = false;
      
      # Add route to NATS container
      networking.extraHosts = ''
        10.233.1.2 nats
      '';
    };
  };

  # Example: CIM-Domain Container (when available)
  # containers.cim-domain = {
  #   autoStart = true;
  #   privateNetwork = true;
  #   hostAddress = "10.233.3.1";
  #   localAddress = "10.233.3.2";
  #
  #   config = { config, pkgs, ... }: {
  #     imports = [
  #       (builtins.getFlake "github:thecowboyai/cim-domain").nixosModules.default
  #     ];
  #
  #     services.cim-domain = {
  #       enable = true;
  #       natsUrl = "10.233.1.2:4222";
  #     };
  #
  #     boot.isContainer = true;
  #     system.stateVersion = "23.11";
  #     networking.useDHCP = false;
  #   };
  # };

  # Host networking configuration
  networking.nat = {
    enable = true;
    internalInterfaces = ["ve-+"];
    externalInterface = "eth0";  # Adjust to your external interface
  };

  # Enable IP forwarding
  boot.kernel.sysctl = {
    "net.ipv4.ip_forward" = 1;
  };

  # Optional: Monitoring stack
  # containers.prometheus = {
  #   autoStart = true;
  #   privateNetwork = true;
  #   hostAddress = "10.233.9.1";
  #   localAddress = "10.233.9.2";
  #
  #   config = { config, pkgs, ... }: {
  #     services.prometheus = {
  #       enable = true;
  #       scrapeConfigs = [
  #         {
  #           job_name = "nats";
  #           static_configs = [{
  #             targets = [ "10.233.1.2:7777" ];
  #           }];
  #         }
  #       ];
  #     };
  #
  #     boot.isContainer = true;
  #     system.stateVersion = "23.11";
  #   };
  # };
}