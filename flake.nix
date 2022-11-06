{

  nixConfig = {
    extra-substituters = [
      "https://nix-community.cachix.org"
    ];
    extra-trusted-public-keys = [
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , naersk
    , fenix
    }:
    let
      supportedSystems = [
        "aarch64-linux"
        "aarch64-darwin"
        "i686-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];
    in
    flake-utils.lib.eachSystem supportedSystems (system:
    let
      lib = nixpkgs.lib;
      pkgs = nixpkgs.legacyPackages.${system};
      fenix-pkgs = fenix.packages.${system};
      rust = with fenix-pkgs; combine [
        default.toolchain
        targets.wasm32-unknown-unknown.latest.toolchain
      ];
      # Get a naersk with the input rust version
      naerskWithRust = rust: naersk.lib."${system}".override {
        rustc = rust;
        cargo = rust;
      };
      env = with pkgs; {
        LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
        PROTOC = "${protobuf}/bin/protoc";
        PROTOBUF_ROOT_DIR = "${protobufc.out}";
        ROCKSDB_LIB_DIR = "${rocksdb}/lib";
        OPENSSL_LIB_DIR = "${openssl.out}/lib";
        OPENSSL_ROOT_DIR = "${openssl.out}";
        OPENSSL_INCLUDE_DIR = "${openssl.dev}/include";
      };
      # Naersk using the default rust version
      buildRustProject = pkgs.makeOverridable ({ rust, naersk ? naerskWithRust rust, ... } @ args: naersk.buildPackage ({
        buildInputs = with pkgs; [ clang openssl ];
        targets = [ ];
        copyLibs = true;
        copyBins = true;
        copySources = [ "src" ];
        cargoBuildOptions = d: d;
        remapPathPrefix =
          true; # remove nix store references for a smaller output package
      } // env // args));

      # Load a nightly rust. The hash takes precedence over the date so remember to set it to
      # something like `lib.fakeSha256` when changing the date.
      crateName = "rustyverse";
      root = ./.;
      # This is a wrapper around naersk build
      # Remember to add Cargo.lock to git for naersk to work
      project = buildRustProject {
        inherit root rust;
      };
      # Running tests
      testProject = project.override {
        doCheck = true;
      };
    in
    {
      packages = {
        ${crateName} = project;
        "${crateName}-test" = testProject;
        default = self.packages.${system}.${crateName};
      };

      apps = {
        server = {
          type = "app";
          program =
            "${self.packages.${system}.${crateName}}/bin/server";
        };
        cli = {
          type = "app";
          program =
            "${self.packages.${system}.${crateName}}/bin/cli";
        };
        default = self.apps.${system}.server;
      };


      # `nix develop`
      devShells = {
        default = pkgs.mkShell (env // {
          inputsFrom = builtins.attrValues self.packages.${system};
          nativeBuildInputs = [ rust ];
          buildInputs = with pkgs; [
            rust-analyzer
            clippy
            rustfmt
          ];
          RUST_BACKTRACE = 1;
          RUST_LOG = "info";
        });
      };
    });
}
