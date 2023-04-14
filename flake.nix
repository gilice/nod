{
  inputs.flake-utils.url = "github:numtide/flake-utils";
  outputs =
    { self
    , nixpkgs
    , flake-utils
    }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = "nod";
        version = "0.1.0";
        src = ./.;

        cargoSha256 = "sha256-mMPBtqWLEDWAdThxM7hEWPmLKca2sQPjgRrRRetyt7k=";
      };
    });
}
