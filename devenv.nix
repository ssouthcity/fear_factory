{ pkgs, config, ... }:

{
  packages = [
    # rust
    pkgs.pkg-config

    # bevy
    pkgs.libudev-zero
    pkgs.alsa-lib
    pkgs.vulkan-loader
    pkgs.vulkan-tools
    pkgs.libx11
    pkgs.libxcursor
    pkgs.libxrandr
    pkgs.libxi
    pkgs.libxkbcommon
    pkgs.wayland

    # github actions
    pkgs.act

    # docs
    pkgs.mdbook

    # profiling
    pkgs.tracy
  ];

  languages.rust = {
    enable = true;
    channel = "nightly";
    mold.enable = true;
  };

  env.LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath config.packages;

  treefmt = {
    enable = true;
    config.programs = {
      nixfmt.enable = true;
      rustfmt.enable = true;
      taplo.enable = true;
    };
  };

  git-hooks.hooks = {
    # General
    trim-trailing-whitespace.enable = true;
    end-of-file-fixer.enable = true;

    # Rust
    cargo-check.enable = true;
    clippy.enable = true;

    # Formatters
    treefmt.enable = true;

    # Git
    check-merge-conflicts.enable = true;
  };
}
