{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  
  outputs = { self, nixpkgs }: {
    devShells.aarch64-darwin.default = 
      let pkgs = nixpkgs.legacyPackages.aarch64-darwin;
      in pkgs.mkShell {
        nativeBuildInputs = [ pkgs.pkg-config ];
        buildInputs = [ pkgs.openssl ];
      };
  };
}
