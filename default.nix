 with (import <nixpkgs> {});

rustPlatform.buildRustPackage rec {
  name = "nix-manifest-format-${version}";
  version = "0.3.0"; # ??? how to link this to Cargo.toml ???

  src = fetchFromGitHub {
    owner = "allgreed";
    repo = "nix-manifest-format";
    rev = "master"; # TODO: This is crap - start using tags to mark versions
    sha256 = "1bm13gwvhbdhljnnsfprfkac0ahgq68xn8wx84xrjnn88pyj2x34";
  };

  cargoSha256 = "1i5q0asq4k2hv07f6253yrwbcj22msxv6lagx4fz054sjn1m34j6";

  #meta = with stdenv.lib; {
  #  description = "A fast line-oriented regex search tool, similar to ag and ack"; # ???
  #  homepage = https://github.com/BurntSushi/ripgrep;
  #  license = licenses.unlicense; # ???
  #  maintainers = [ maintainers.tailhook ]; # ???
  #  platforms = platforms.all; # ???
  #};
}
