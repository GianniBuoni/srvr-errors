# module defines shell congfigurations shared by all shells
{moduleWithSystem, ...}: {
  flake.aspects.devshells.base = moduleWithSystem ({pkgs, ...}: {
    packages = with pkgs; [just];
  });
}
