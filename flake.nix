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
      # rust-analyzer は rustc と proc-macro の ABI が一致している必要があるため、
      # nixpkgs 版ではなく同じツールチェーンの extension から取る
      # (ずれると #[fastout] の展開で proc-macro-srv is not running になる)
      rustToolchain = pkgs.rust-bin.stable."1.89.0".default.override {
        extensions = [ "rust-src" "rust-analyzer" ];
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
      # nixpkgs に無いため crates.io からビルド (CLI は binaries feature が必須)
      cargo-snippet = pkgs.rustPlatform.buildRustPackage rec {
        pname = "cargo-snippet";
        version = "0.6.5";
        src = pkgs.fetchCrate {
          inherit pname version;
          hash = "sha256-zgNwNcWMcI3w4wh9i7JyidcG0XVjA1Cd8B9sD9kPLWA=";
        };
        cargoHash = "sha256-zeBvyeOEeDYU2Iw9D9CDfSfAoAz/q+46148GnC8ZNMw=";
        buildFeatures = [ "binaries" ];
        doCheck = false;
      };
    in
    {
      packages.${system} = { inherit cargo-compete cargo-snippet; };

      devShells.${system}.default = pkgs.mkShell {
        packages = [
          rustToolchain
          cargo-compete
          cargo-snippet
        ];
      };
    };
}
