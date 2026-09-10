{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        packages.default = naersk-lib.buildPackage ./.;

        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/cangaroo-cli";
        };
        
        devShells.default = with pkgs; mkShell {
          buildInputs = [ cargo rustc rustfmt rust-analyzer pre-commit rustPackages.clippy pkg-config udev ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      }
    );
}
