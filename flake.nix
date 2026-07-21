{
  description = "Terminal disk space visual navigator";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "diskonaut";
          version = "0.11.0";

          src = pkgs.lib.cleanSource ./.;

          cargoLock.lockFile = ./Cargo.lock;

          meta = with pkgs.lib; {
            description = "Terminal disk space visual navigator";
            homepage = "https://github.com/imsnif/diskonaut";
            license = licenses.mit;
            mainProgram = "diskonaut";
          };
        };

        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.default;
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ cargo rustc rustfmt clippy rust-analyzer ];
        };
      });
}
