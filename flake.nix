{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/24.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay,... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let 
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in rec {
        packages = flake-utils.lib.flattenTree {
          asteroids = pkgs.callPackage ./default.nix {};
        };
        defaultPackage = packages.asteroids;
        apps.asteroids = flake-utils.lib.mkApp {
          drv = packages.asteroids;
          exePath = "/bin/asteroids";
        };
        apps.default = apps.asteroids;
        defaultApp = apps.asteroids;
        devShell = pkgs.mkShell {
          name = "asteroids";
          buildInputs = with pkgs; [
            rust-bin.beta.latest.default
            rust-analyzer
            SDL2
          ];
          shellHook = "";
        };
      });
}
