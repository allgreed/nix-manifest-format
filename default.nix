 with (import <nixpkgs> {});

rustPlatform.buildRustPackage rec {
  name = "nix-manifest-format-${version}";
  version = "0.3.1";

  src = fetchFromGitHub {
    owner = "allgreed";
    repo = "nix-manifest-format";
    rev = "${version}";
    sha256 = "01kwb4my0p0p75pa0rlkx3lagly0ng7yfqx4lf27r8h02y4lnf5w";
  };

  cargoSha256 = "1i5q0asq4k2hv07f6253yrwbcj22msxv6lagx4fz054sjn1m34j6";

  meta = with stdenv.lib; {
    description = "A utility for formatting and removing metadata from manifest.nix";
    homepage = https://github.com/allgreed/nix-manifest-format/;
    license = licenses.mit;
    maintainers = [ { github = "allgreed"; name = "Olgierd Kasprowicz"; } ];
    platforms = platforms.all;
  };
}
