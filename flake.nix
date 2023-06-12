{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = {self, nixpkgs, fenix, naersk, flake-utils }:
  flake-utils.lib.eachDefaultSystem(system:
  let
    pkgs = import nixpkgs {
      inherit system;
    };
    rustToolchain = fenix.packages.${system}.fromToolchainFile {
      dir = ./.;
      sha256 = pkgs.lib.fakeSha256;
    };
    naersk-lib = naersk.lib.${system}.override {
      cargo = rustToolchain;
      rustc = rustToolchain;
    };
    nativeBuildInputs = with pkgs; [
      pkg-config
      rustToolchain
    ];
    buildInputs = with pkgs; [
      openssl
    ];
    bin = naersk-lib.buildPackage {
      src = ./.;
      doCheck = true;
      pname = "clinews";
      version = "0.1.0";
      inherit nativeBuildInputs buildInputs;
    };
  in
  {
    packages = {
      inherit bin;
      default = bin;
    };
  });
}
