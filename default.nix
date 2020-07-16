{
  system ? builtins.currentSystem

  # Rust stuff
, nixpkgsMozilla ? builtins.fetchGit {
  url = https://github.com/mozilla/nixpkgs-mozilla;
  ref = "master";
}
, cargo2nix ? builtins.fetchGit {
  url = https://github.com/tenx-tech/cargo2nix;
  ref = "master";
}

}:

let
  rustOverlay = import "${nixpkgsMozilla}/rust-overlay.nix";
  cargo2nixOverlay = import "${cargo2nix}/overlay";

  pkgs = import <nixpkgs> {
    inherit system;
    overlays = [ rustOverlay cargo2nixOverlay ];
  };

  rustPkgs = pkgs.rustBuilder.makePackageSet' {
    rustChannel = "nightly";
    packageFun = import ./Cargo.nix;
  };
in
(rustPkgs.workspace.vcpu {}).overrideAttrs (
  old: rec {
    buildInputs = old.buildInputs ++ [
      pkgs.llvmPackages.libclang
    ];

    configurePhase =
      with pkgs.llvmPackages;
      old.configurePhase + ''
        export LIBCLANG_PATH="${libclang}/lib"
      '';

    shellHook =
      with pkgs.llvmPackages;
      ''
        export LIBCLANG_PATH="${libclang}/lib"
      '';
  }
)
