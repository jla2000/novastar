{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      graphicsPackages = with pkgs; [
        libGL
        vulkan-loader
        libxkbcommon
        wayland
        xorg.libX11
        xorg.libXcursor
        xorg.libXi
        xorg.libXrandr
        xorg.libXinerama
      ];
      libPath = pkgs.lib.makeLibraryPath graphicsPackages;
    in
    {
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "novastar";
        version = "0.1.0";
        src = self;
        cargoLock = {
          lockFile = ./Cargo.lock;
        };
        postFixup = ''
          patchelf --add-rpath ${libPath} $out/bin/novastar
        '';
      };
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [ cmake rustc cargo ];
        LD_LIBRARY_PATH = libPath;
        buildInputs = graphicsPackages;
        RUST_LOG = "info";
      };
    };
}
