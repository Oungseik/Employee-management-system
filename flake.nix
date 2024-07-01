{
  inputs = {
    utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, utils }: utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShell = pkgs.mkShellNoCC {
        buildInputs = with pkgs; [
          cargo-audit
          cargo-nextest
          sqlx-cli
          cargo-tarpaulin
          cargo-watch

          # optional dependencies to make RESTapi request from Neovim
          luajitPackages.lua-curl
          luajitPackages.nvim-nio
          luajitPackages.mimetypes
          luajitPackages.xml2lua
          pkg-config
          openssl
        ];

        STATIC_ASSETS_FOLDER = "static";
        DATABASE_URL = "sqlite://database/db.sqlite";
        RUSTUP_TOOLCHAIN="stable";
      };
    }
  );
}
