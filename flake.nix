{
  inputs = {
    utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, utils }: utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          cargo-audit
          cargo-watch
          cargo-tarpaulin

          # optional dependencies to make RESTapi request from Neovim
          luajitPackages.lua-curl
          luajitPackages.nvim-nio
          luajitPackages.mimetypes
          luajitPackages.xml2lua
          pkg-config
          openssl
        ];
      };
    }
  );
}
