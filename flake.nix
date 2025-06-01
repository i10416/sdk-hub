{
  description = "SDK Hub";
  inputs.nixpkgs.url = "nixpkgs/nixpkgs-unstable";
  inputs.flake-parts.url = "github:hercules-ci/flake-parts";
  inputs.rust-overlay = {
    url = "github:oxalica/rust-overlay";
    inputs.nixpkgs.follows = "nixpkgs";
  };
  outputs =
    inputs@{
      self,
      nixpkgs,
      flake-parts,
      rust-overlay,
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      perSystem =
        { pkgs, system, ... }:
        let
          overlays = [
            (import rust-overlay)
          ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          toolchain = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain).override {
            extensions = [ "rust-src" ];
          };

          sharedBuildInputs = with pkgs; [
            toolchain
          ];
        in
        {
          devShells = {
            default = pkgs.mkShell {
              buildInputs = sharedBuildInputs;
            };
          };
        };
    };
}
