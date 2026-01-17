{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs =
    inputs:
    let
      system = "x86_64-linux";
      overlays = [ inputs.rust-overlay.overlays.default ];
      pkgs = import inputs.nixpkgs { inherit system overlays; };
      rust-dev = (
        pkgs.rust-bin.selectLatestNightlyWith (
          toolchain:
          toolchain.minimal.override {
            extensions = [
              "rust-analyzer"
              "rust-src"
              "rustfmt"
            ];
          }
        )
      );
      lib = pkgs.lib;
    in
    {
      nixosModules.default =
        { config, ... }:
        let
          cfg = config.niri-ws;
        in
        {
          options.niri-ws = {
            enable = lib.mkEnableOption "niri-ws";
          };
          config = lib.mkIf cfg.enable {
            environment.systemPackages = [ inputs.self.packages.${system}.default ];
          };
        };
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "niri-ws";
        version = "0.1.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
        meta = {
          description = "A tool to swap workspaces between outputs in Niri";
          homepage = "https://github.com/rossnomann/niri-ws";
          license = lib.licenses.mit;
          mainProgram = "niri-ws";
        };
      };
      devShells.${system}.default = pkgs.mkShell {
        RUST_SRC_PATH = "${rust-dev}/lib/rustlib/src/rust/library";
        buildInputs = [
          pkgs.pkg-config
          (pkgs.lib.hiPrio (
            pkgs.rust-bin.stable.latest.minimal.override {
              extensions = [
                "rust-docs"
                "clippy"
              ];
            }
          ))
          rust-dev
        ];
        shellHook = ''
          export CARGO_HOME="$PWD/.cargo"
          export PATH="$CARGO_HOME/bin:$PATH"
          export RUST_LOG=info
          mkdir -p .cargo
          echo '*' > .cargo/.gitignore
        '';
      };
    };
}
