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
        defaultPackage = naersk-lib.buildPackage {
          src = ./.;
          nativeBuildInputs = [ pkgs.capnproto ];  # Add capnproto here
        };
        devShell = with pkgs; mkShell {
          buildInputs = [ 
            cargo 
            rustc 
            rustfmt 
            capnproto  # Available in dev shell
            pre-commit 
            rustPackages.clippy 
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      }
    );
}

