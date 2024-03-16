{ pkgs ? import <nixpkgs> { }}:

pkgs.mkShell {
  name = "RUST DEV";
  packages = [
  ];

  nativeBuildInputs = (with pkgs; [
  ]);

  shellHook = ''
    # check if distrobox is installed
    if ! command -v distrobox &> /dev/null; then
      echo "distrobox is not installed, installing it now"
    else
      echo "distrobox is installed"
      echo "Entering to aur"
      distrobox-enter aur
    fi
  '';
}

