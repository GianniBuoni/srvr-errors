# module to define build inputs for the shells and nix-build
{moduleWithSystem, ...}: {
  flake.aspects.devshells.build = moduleWithSystem ({pkgs, ...}: {
    packages = with pkgs; [sqlx-cli postgresql pkg-config];

    env = [
      {
        name = "PGDATA";
        value = ".postgres";
      }
      {
        name = "DATABASE_URL";
        value = "postgres://[::1]:5432";
      }
    ];

    commands = [
      {
        name = "enterTest";
        help = "Test build shell has all necessary tooling";
        command = ''
          cargo -V
          cargo clippy -V
          just -V
          postgres -V
          sqlx -V
        '';
      }
    ];
  });
}
