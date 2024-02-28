{ pkgs ? import <nixpkgs-unstable> {} }:

# nix-shell
pkgs.mkShell {
    nativeBuildInputs = with pkgs.buildPackages; [
      pkgs.pkg-config
      pkgs.openssl
      pkgs.rustc
      pkgs.cargo
    ];
}