let pkgs = import <nixpkgs> {};
in pkgs.rustPlatform.buildRustPackage {
	pname = "pitr_info";
	version = "0.1.0";

	src = ./.;

	cargoSha256 = "0dz3ng2cfp5gggdcjgfwspswl4l1fjdvmpwpb8gmp8vq3gx8p49z";
}
