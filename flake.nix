{
  description = "AtCoder competitive programming environment (Rust)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "aarch64-darwin";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
      # AtCoder のジャッジ環境 (2025/10 言語アップデート) と同じバージョンに固定
      rustToolchain = pkgs.rust-bin.stable."1.89.0".default.override {
        extensions = [ "rust-src" ];
      };
      # nixpkgs に無いため crates.io からビルド
      cargo-compete = pkgs.rustPlatform.buildRustPackage rec {
        pname = "cargo-compete";
        version = "0.10.7";
        src = pkgs.fetchCrate {
          inherit pname version;
          hash = "sha256-EbseENvy8vBn97aR2LlH8eVgHtC1DKECSo6Dw0X6Vo0=";
        };
        cargoHash = "sha256-lid1tyR8Y6lvjpeGJ4vGzqDTY6V2y/5rL9fGyjyF3yw=";
        # 0.10.7 は edition 2024 (現ジャッジの edition) を config で受け付けないため追加
        postPatch = ''
          substituteInPlace src/config.rs --replace-fail \
            'Edition2021,' \
            'Edition2021, #[strum(serialize = "2024")] Edition2024,'
        '';
        nativeBuildInputs = [ pkgs.pkg-config ];
        buildInputs = [ pkgs.openssl pkgs.zlib ];
        doCheck = false;
      };
    in
    {
      packages.${system}.cargo-compete = cargo-compete;

      devShells.${system}.default = pkgs.mkShell {
        packages = [
          rustToolchain
          cargo-compete
          pkgs.rust-analyzer
        ];
      };
    };
}
