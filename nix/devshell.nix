# module to define all devshell imports
{
  inputs,
  config,
  ...
}: {
  flake-file.inputs.devshell.url = "github:numtide/devshell";
  imports = [inputs.devshell.flakeModule];

  perSystem = {self', ...}: {
    devShells.default = self'.devShells.dev;
    devshells = with config.flake.aspects.devshells; {
      # development shell with full tooling
      dev = {extraModulesPath, ...}: {
        imports = [
          "${extraModulesPath}/language/rust.nix"
          "${extraModulesPath}/language/c.nix"
          "${extraModulesPath}/git/hooks.nix"
          base
          build
          dev
          gitHooks
        ];
      };
      # ci shell with only the tooling necessary for linting
      lint = {extraModulesPath, ...}: {
        imports = [
          "${extraModulesPath}/language/rust.nix"
          "${extraModulesPath}/language/c.nix"
          base
          lint
        ];
      };
      # ci shell with only the tooling necessary for testing and building
      build = {extraModulesPath, ...}: {
        imports = [
          "${extraModulesPath}/language/rust.nix"
          "${extraModulesPath}/language/c.nix"
          base
          build
        ];
      };
    };
  };
}
