# niri-ws

A tool to swap workspaces between outputs in Niri.

## Installation

### NixOS

`flake.nix`:
```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    niri-ws = {
      url = "github:rossnomann/niri-ws";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = inputs: {
    nixosConfigurations.default = inputs.nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        inputs.niri-ws.nixosModules.default
        (
          { ... }:
          {
            config = {
              niri-ws.enable = true;
            };
          }
        )
      ];
  };
}
```

## Configuration

Add the following bind to your Niri config:

```kdl
binds {
  Mod+W {
    spawn "/path/to/niri-ws";
  }
}
```

## LICENSE

The MIT License (MIT)
