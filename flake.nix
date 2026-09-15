{
  description = "A sl but with any text you chose.";

  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.zst";
  };

  outputs = inputs: {
    packages = builtins.mapAttrs (system: pkgs: {

      default = pkgs.rustPlatform.buildRustPackage {
        pname = "custom-sl";
        version = "0.1.0";

        cargoHash = "sha256-nQXiZQ1fPXNQlZFfEut4CeMkUBEwi8tfhOQAoqe7qa8=";

        src = ./.;

        nativeBuildInputs = [ pkgs.pkg-config ];

        meta = {
          description = "A sl but with any text you chose.";
          homepage = "https://github.com/itsvyle/custom-sl";
          mainProgram = "custom-sl";
        };
      };
    }) inputs.nixpkgs.legacyPackages;
  };
}
