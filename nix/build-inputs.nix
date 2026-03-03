# module to define build inputs for the shells and nix-build
{moduleWithSystem, ...}: {
  flake.aspects.devshells.build = moduleWithSystem ({pkgs, ...}: {
    packages = with pkgs; [pkg-config];
  });
}
