with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "gncrt-shell";

  buildInputs = with pkgs; [
    rustc
    cargo

    llvmPackages.libclang

    pkgconfig

    zlib
  ];

  shellHook = ''
export LIBCLANG_PATH="${llvmPackages.libclang}/lib"
  '';
}
