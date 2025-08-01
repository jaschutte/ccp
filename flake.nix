{
  description = "Rust Development";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs @ { self, nixpkgs, flake-utils, ... }:
  flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
      };

      ccp = pkgs.rustPlatform.buildRustPackage {
        pname = "ccp";
        version = "0.1.0";
        src = ./.;

        cargoLock.lockFile = ./Cargo.lock;
      };
    in {
      packages.default = ccp;

      apps = {
        ccopy = {
          type = "app";
          program = "${ccp}/bin/ccopy";
        };
        cpaste = {
          type = "app";
          program = "${ccp}/bin/cpaste";
        };
      };

      devShells.default = pkgs.mkShell {
        buildInputs = [
          pkgs.cargo
          pkgs.clippy
        ];
      };
    }
  );
}
