{
  inputs = { nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable"; };

  outputs = { self, nixpkgs }:
    let
      systems =
        [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = fn:
        nixpkgs.lib.genAttrs systems
        (system: fn { pkgs = import nixpkgs { inherit system; }; });
    in {
      devShells = forAllSystems ({ pkgs }: {
        default = pkgs.mkShell {
          name = "rust";
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = with pkgs; [ cargo clippy rust-analyzer rustc rustfmt ];
        };
      });
      formatter = forAllSystems ({ pkgs }: pkgs.nixfmt-classic);
    };
}
