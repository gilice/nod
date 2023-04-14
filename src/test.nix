let pkgs = import <nixpkgs> { };
in pkgs.stdenv.mkDerivation {
  outputHash = pkgs.lib.fakeSha256;
  buildPhase = ''
    # fill with 2^20 bytes (1MB) of random data
    fallocate --length 1M $out
    openssl rand -out $out -base64 $(( 2**20 * 3/4 ))
  '';
  src = ./.;
  name = "foo";
  outputHashAlgo = "sha256";
  nativeBuildInputs = with pkgs;[ openssl utillinux ];
}
