{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";

    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
    utils,
    ...
  } @ inputs:
    utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      devShell = pkgs.mkShell {
        nativeBuildInputs = [
          pkgs.dioxus-cli
          pkgs.tailwindcss_4
          pkgs.wasm-bindgen-cli_0_2_100
          # fenix.packages.${system}.stable.toolchain
          # (with fenix.packages.${system};
          #   combine [
          #     minimal.cargo
          #     minimal.rustc
          #     targets.wasm32-unknown-unknown.latest.rust-std
          #   ])
        ];
        RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
      };
    });
}
