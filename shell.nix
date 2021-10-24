{ pkgs ? import <nixpkgs> {} }:
with pkgs;
let llvmPackages = pkgs.llvmPackages_latest;
    libclang = llvmPackages.libclang;
in mkShell {
  buildInputs = [ pkgconfig libclang libudev openssl ];
  LIBCLANG_PATH = "${libclang}/lib";
}
