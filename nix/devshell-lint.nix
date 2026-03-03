# module to define shell configurations exclusive to the linting shell
{
  flake.aspects.devshells.lint.commands = [
    {
      name = "enterTest";
      help = "Test to check lint shell has all necessary tooling";
      command = ''
        cargo -V
        cargo clippy -V
        just -V
      '';
    }
  ];
}
