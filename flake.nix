{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/*.tar.gz";
    parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ self, nixpkgs, parts }:
    parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      perSystem = { system, ... }:
        let
          pkgs = import nixpkgs { inherit system; };
          dev_packages = with pkgs; [
            pkg-config
            cargo-deny
            cargo-edit
            cargo-watch
            rust-analyzer
            rustfmt          
          ];
        in
        {
          devShells.default = pkgs.mkShell {
            packages = dev_packages;
         };
        };
    };
}
