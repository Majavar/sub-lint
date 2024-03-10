{
  description = "Subtitle linter";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.nightly."2024-03-09".minimal.override {
          extensions = [ "rustfmt" "clippy" "rust-src" ];
        };
      in
      with pkgs;
      {
        devShells.default = mkShell.override { stdenv = stdenvNoCC; } {
          buildInputs = [
            rust
          ];
        };
      }
    );
}
