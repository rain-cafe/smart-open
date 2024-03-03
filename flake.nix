{
  description = "smart-open";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { nixpkgs, ... }:
    let
      inherit (nixpkgs) lib legacyPackages;
      forAllSystems = lib.genAttrs lib.systems.flakeExposed;
    in
    {
      overlay = final: prev: {
        smart-open = prev.callPackage ./nix/smart-open.nix { };
      };

      packages = forAllSystems (system:
        let
          pkgs = legacyPackages.${system};
        in
        {
          default = pkgs.callPackage ./nix/smart-open.nix { };
        });
    };
}
