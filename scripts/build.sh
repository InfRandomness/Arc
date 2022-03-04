pushd kernel/
  cargo build
popd || exit

pushd xtask/
  cargo xtask image ../target/x86_64-arc/debug/kernel
popd || exit