{inputs, ...}: {
  flake-file.inputs = {
    # nixpkgs
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    import-tree.url = "github:vic/import-tree";
    # dentrtic
    flake-aspects.url = "github:vic/flake-aspects";
    flake-file.url = "github:vic/flake-file";
    flake-parts.url = "github:hercules-ci/flake-parts";
    # systems
    systems = {
      url = "github:nix-systems/default";
      flake = false;
    };
  };
  # import dendritic modules
  imports = with inputs; [
    flake-file.flakeModules.default
    flake-file.flakeModules.nix-auto-follow
    flake-aspects.flakeModule
  ];
  # declare outputs function
  flake-file.outputs = "inputs: inputs.flake-parts.lib.mkFlake {inherit inputs;} (inputs.import-tree ./nix)";
  # use systems library to define flake-parts systems
  systems = import inputs.systems;
}
