{
  nixpkgs ? import <nixpkgs> { },
}:
nixpkgs.mkShell {
  buildInputs = with nixpkgs; [
    cargo
    rustc
  ];
}
