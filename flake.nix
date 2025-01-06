{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShell = with pkgs; mkShell {
          packages = [
            valgrind-light
            clippy
            wasm-bindgen-cli
          ];

          buildInputs = [
            cargo
            rustc
            rustfmt
            pre-commit
            pkg-config

            gobject-introspection
            atkmm
            pango
            gtk3-x11

            openssl_3_3
          ];
          nativeBuildInputs = [
            # llvmPackages_12.lld
            # llvmPackages_12.libcxxClang
          ];
          LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${
            lib.makeLibraryPath [
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              libxkbcommon
              xorg.libxcb
              vulkan-loader
            ]
          }";

          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          # RUSTFLAGS = "-C linker=lld";
          # CC = "${llvmPackages_12.libcxxClang}";
        };
      }
    );
}
