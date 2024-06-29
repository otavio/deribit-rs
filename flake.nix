{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    pre-commit-hooks-nix = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        nixpkgs-stable.follows = "nixpkgs";
      };
    };

    nix-github-actions = {
      url = "github:nix-community/nix-github-actions";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-flake = {
      url = "github:juspay/rust-flake";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ flake-parts, ... }:
    let
      inherit (inputs.nixpkgs) lib;
      inherit (inputs) self;
    in
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.pre-commit-hooks-nix.flakeModule
        inputs.rust-flake.flakeModules.default
        inputs.rust-flake.flakeModules.nixpkgs
      ];

      systems = lib.systems.flakeExposed;

      flake.githubActions = inputs.nix-github-actions.lib.mkGithubMatrix {
        checks = lib.getAttrs [ "x86_64-linux" ] self.checks;
      };

      perSystem = { self', config, pkgs, ... }: {
        pre-commit.settings = {
          hooks = {
            convco.enable = true;
            nixpkgs-fmt.enable = true;
            rustfmt.enable = true;
          };
        };

        packages.default = self'.packages.deribit;

        devShells.default = pkgs.mkShell {
          inputsFrom = [
            config.pre-commit.devShell
            self'.devShells.deribit
          ];

          packages = with pkgs; [
            cargo
            cargo-watch
            cargo-release
            rust-analyzer
            rustc
          ];
        };
      };
    };
}
