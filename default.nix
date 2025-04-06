{ pkgs ? import <nixpkgs> {}}:
pkgs.rustPlatform.buildRustPackage
rec {
  pname = "asteroids";
  version = "1.0";
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;
  buildInputs = with pkgs; [
    SDL2
  ];
  meta = with pkgs.lib; {
    description = "Asteroids game, built with rust";
    homepage = "https://github.com/carlosdanielmaturano/asteroids";
    license = licenses.mit;
    maintainers = [];
  };
}
