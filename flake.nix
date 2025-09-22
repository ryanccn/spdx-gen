# SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
#
# SPDX-License-Identifier: GPL-3.0-only

{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    ferrix.url = "github:ryanccn/ferrix";
  };

  outputs =
    { ferrix, ... }@inputs:
    ferrix.lib.mkFlake inputs {
      root = ./.;
      completions = {
        enable = true;
        args = "--completions";
      };
    };
}
