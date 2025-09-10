{ pkgs, config, ... }:

{
  packages = [
    pkgs.pkg-config

    pkgs.udev
    pkgs.alsa-lib-with-plugins
    pkgs.vulkan-loader

    # x11
    pkgs.xorg.libX11
    pkgs.xorg.libXcursor
    pkgs.xorg.libXrandr
    pkgs.xorg.libXi

    # wayland
    pkgs.libxkbcommon
    pkgs.wayland

    # github actions
    pkgs.act
  ];

  languages.rust = {
    enable = true;
    channel = "nightly";
    mold.enable = true;
  };

  env.LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath config.packages;

  git-hooks.hooks = {
    # General
    trim-trailing-whitespace.enable = true;
    end-of-file-fixer.enable = true;

    # Rust
    cargo-check.enable = true;
    clippy.enable = true;
    rustfmt.enable = true;

    # Git
    check-merge-conflicts.enable = true;

    # Git LFS
    lfs-post-merge = {
      enable = true;
      name = "git-lfs-post-merge";
      entry = "git lfs post-merge";
      always_run = true;
      pass_filenames = false;
      stages = ["post-merge"];
    };
    lfs-pre-push = {
      enable = true;
      name = "git-lfs-pre-push";
      entry = "git lfs pre-push";
      always_run = true;
      pass_filenames = false;
      stages = ["pre-push"];
    };
    lfs-post-checkout = {
      enable = true;
      name = "git-lfs-post-checkout";
      entry = "git lfs post-checkout";
      always_run = true;
      pass_filenames = false;
      stages = ["post-checkout"];
    };
    lfs-post-commit = {
      enable = true;
      name = "git-lfs-post-commit";
      entry = "git lfs post-commit";
      always_run = true;
      pass_filenames = false;
      stages = ["post-commit"];
    };
  };
}
