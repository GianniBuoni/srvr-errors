# module to define shell configurations exclusive to the development shell
{moduleWithSystem, ...}: {
  flake.aspects.devshells.dev = moduleWithSystem ({pkgs, ...}: {
    packages = with pkgs; [sqlx-cli commitizen rust-analyzer];
  });
}
