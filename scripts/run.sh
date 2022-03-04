pushd kernel/
  cargo build
popd || exit

pushd xtask/
  cargo xtask image ../target/x86_64-arc/debug/kernel
popd || exit

qemu-system-x86_64 -drive format=raw,file=target/x86_64-arc/debug/boot-uefi-kernel.img -bios /usr/share/qemu/ovmf-x86_64.bin