{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
    asciidocProcessor = import ./processors/asciidoc { inherit pkgs; };
    pandocProcessor = import ./processors/pandoc { inherit pkgs; };

  in {
    devShell."${system}" = pkgs.mkShell {
      buildInputs = [
        asciidocProcessor
        pandocProcessor
      ];
    };

    packages."${system}".eighty = let
      rustPkg = pkgs.rustPlatform.buildRustPackage rec {
        pname = "eighty";
        version = "0.1.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
      };

    in pkgs.writeShellApplication {
      name = "eighty";
      text = ''
        ${rustPkg}/bin/eighty "$@"
      '';
      runtimeInputs = [ asciidocProcessor pandocProcessor ];
    };

    apps."${system}".default = {
      type = "app";
      program = "${self.packages."${system}".eighty}/bin/eighty";
    };
  };
}
