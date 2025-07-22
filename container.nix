# Copyright (c) 2025 Cowboy AI, LLC.
#
# NixOS container configuration for CIM-Subject service
#
# Usage with extra-container:
#   sudo extra-container create --start < container.nix
#
# Usage with nixos-container:
#   sudo nixos-container create cim-subject --config-file container.nix
#   sudo nixos-container start cim-subject

{ config, pkgs, lib, ... }:

{
  # Container configuration
  boot.isContainer = true;
  networking.useDHCP = false;
  networking.hostName = "cim-subject";

  # Import the CIM-Subject module
  imports = [
    (builtins.getFlake "github:thecowboyai/cim-subject").nixosModules.default
  ];

  # Configure the CIM-Subject service
  services.cim-subject = {
    enable = true;
    natsUrl = "10.233.1.1:4222";  # Adjust to your NATS server
    logLevel = "info";
  };

  # Network configuration
  networking.interfaces.eth0 = {
    useDHCP = false;
    ipv4.addresses = [{
      address = "10.233.2.10";  # Adjust to your network
      prefixLength = 24;
    }];
  };

  networking.defaultGateway = "10.233.1.1";
  networking.nameservers = [ "8.8.8.8" "8.8.4.4" ];

  # Firewall configuration
  networking.firewall = {
    enable = true;
    # Open ports if the service needs to listen
    # allowedTCPPorts = [ 8080 ];
  };

  # System configuration
  system.stateVersion = "23.11";

  # Minimal container optimizations
  documentation.enable = false;
  programs.command-not-found.enable = false;
  
  # Time zone
  time.timeZone = "UTC";

  # Monitoring and logging
  services.journald.extraConfig = ''
    SystemMaxUse=100M
    MaxRetentionSec=7day
  '';

  # Optional: Prometheus node exporter for monitoring
  # services.prometheus.exporters.node = {
  #   enable = true;
  #   enabledCollectors = [ "systemd" "processes" ];
  # };

  # Users
  users.users.cim-subject = {
    isSystemUser = true;
    group = "cim-subject";
    description = "CIM Subject service user";
  };
  
  users.groups.cim-subject = {};

  # Environment
  environment.systemPackages = with pkgs; [
    # Useful tools for debugging
    htop
    jq
    curl
    dig
  ];
}