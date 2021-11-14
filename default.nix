let pkgs = import <nixpkgs> {};
in pkgs.rustPlatform.buildRustPackage {
	pname = "pitr_info";
	version = "0.1.0";

	src = ./.;

	cargoSha256 = "0rk2zh6k7bpn5ndq728f8850b1jin60zjiyi95cnpq5q4dqfggfy";
}
