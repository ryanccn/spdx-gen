# SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
#
# SPDX-License-Identifier: GPL-3.0-only

{
  lib,
  rustPlatform,
  installShellFiles,
  self,
  enableLTO ? true,
  enableOptimizeSize ? false,
}:
let
  year = builtins.substring 0 4 self.lastModifiedDate;
  month = builtins.substring 4 2 self.lastModifiedDate;
  day = builtins.substring 6 2 self.lastModifiedDate;
in
rustPlatform.buildRustPackage (finalAttrs: {
  pname = finalAttrs.passthru.cargoToml.package.name;
  version = finalAttrs.passthru.cargoToml.package.version + "-unstable-${year}-${month}-${day}";

  src = lib.fileset.toSource {
    root = ../.;
    fileset = lib.fileset.unions [
      ../src
      ../build.rs
      ../Cargo.lock
      ../Cargo.toml
    ];
  };

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  nativeBuildInputs = [
    installShellFiles
  ];

  env =
    lib.optionalAttrs enableLTO {
      CARGO_PROFILE_RELEASE_LTO = "fat";
      CARGO_PROFILE_RELEASE_CODEGEN_UNITS = "1";
    }
    // lib.optionalAttrs enableOptimizeSize {
      CARGO_PROFILE_RELEASE_OPT_LEVEL = "z";
      CARGO_PROFILE_RELEASE_PANIC = "abort";
      CARGO_PROFILE_RELEASE_CODEGEN_UNITS = "1";
      CARGO_PROFILE_RELEASE_STRIP = "symbols";
    };

  doCheck = false;

  preBuild = ''
    export COMPLETIONS_OUT_DIR="$TMPDIR/completions"
  '';

  postInstall = ''
    installShellCompletion --cmd ${finalAttrs.pname} \
      --bash "$TMPDIR/completions/${finalAttrs.pname}.bash" \
      --zsh "$TMPDIR/completions/_${finalAttrs.pname}" \
      --fish "$TMPDIR/completions/${finalAttrs.pname}.fish"
  '';

  passthru = {
    cargoToml = lib.importTOML ../Cargo.toml;
  };

  meta = with lib; {
    description = "SPDX license generator";
    maintainers = with maintainers; [ ryanccn ];
    license = licenses.gpl3Only;
    mainProgram = "spdx-gen";
  };
})
