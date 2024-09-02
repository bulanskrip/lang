# To learn more about how to use Nix to configure your environment
# see: https://developers.google.com/idx/guides/customize-idx-env
{ pkgs, ... }: {
  # Which nixpkgs channel to use.
  channel = "stable-23.11"; # or "unstable"

  # Use https://search.nixos.org/packages to find packages
  packages = [
    pkgs.rustfmt
    pkgs.stdenv.cc
    pkgs.rustup
    # pkgs.go
    # pkgs.python311
    # pkgs.python311Packages.pip
    # pkgs.nodejs_20
    # pkgs.nodePackages.nodemon
  ];

  # Sets environment variables in the workspace
  env = {};
  idx = {
    # Search for the extensions you want on https://open-vsx.org/ and use "publisher.id"
    extensions = [
      # "vscodevim.vim"
      "ginfuru.ginfuru-better-solarized-dark-theme"
      "hbenl.test-adapter-converter"
      "hbenl.vscode-test-explorer"
      "LaurentTreguier.vscode-simple-icons"
      "rust-lang.rust-analyzer"
      "serayuzgur.crates"
      "Swellaby.vscode-rust-test-adapter"
      "tamasfe.even-better-toml"
      "miguelsolorio.fluent-icons"
      "pest.pest-ide-tools"
    ];

    # Enable previews
    previews = {
      enable = true;
      previews = {
        # web = {
        #   # Example: run "npm run dev" with PORT set to IDX's defined port for previews,
        #   # and show it in IDX's web preview panel
        #   command = ["npm" "run" "dev"];
        #   manager = "web";
        #   env = {
        #     # Environment variables to set for your server
        #     PORT = "$PORT";
        #   };
        # };
      };
    };

    # Workspace lifecycle hooks
    workspace = {
      # Runs when a workspace is first created
      onCreate = {
        installRustAndDeps = "read -r -p 'This install script is IMPORTANT, do not close it for any reasons' -t 5 -n 1 -s; rustup toolchain add stable; rustup default stable; cargo install";
        # Example: install JS dependencies from NPM
        # npm-install = "npm install";
      };
      # Runs when the workspace is (re)started
      onStart = {
        # Example: start a background task to watch and re-build backend code
        # watch-backend = "npm run watch-backend";
      };
    };
  };
}
