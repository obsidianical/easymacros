with import <nixpkgs> {};
pkgs.mkShell {
  buildInputs = with pkgs; [ 
    cargo 
    rustc 
    rust-analyzer 
    rustfmt 
    pkg-config
    xorg.libX11
    xorg.libXtst
  ]; 
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
