{ pkgs ? import <nixpkgs> { } }:
let manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = manifest.name;
  version = manifest.version;
  cargoLock.lockFile = ./Cargo.lock;

  cargoLock.outputHashes = {
    "forge-widgets-0.1.0" = "sha256-hON0WLZpcS/ODSVr8ztpxWfjwGgtSc2fRUtv6fxPSps=";
  };

  src = pkgs.lib.cleanSource ./.;
}
