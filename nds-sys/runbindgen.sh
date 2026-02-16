clang_version="11.1.0"


WONDERFUL_ARM="$WONDERFUL_TOOLCHAIN/toolchain/gcc-arm-none-eabi/"
BLOCKSDS="$WONDERFUL_TOOLCHAIN/thirdparty/blocksds/core/"
NDS_H="$BLOCKSDS/libs/libnds/include/nds.h"

~/.cargo/bin/bindgen \
    ./wrapper.h \
    --raw-line "#![allow(warnings)]" \
    --rust-target nightly \
    --use-core \
    --distrust-clang-mangling \
    --no-doc-comments \
    --no-layout-tests \
    --ctypes-prefix "core::ffi" \
    --sort-semantically \
    --blocklist-type "(s|u)(8|16|32|64)" \
    --constified-enum-module "VideoMode" \
    --rustified-enum "VRAM_[A-I]_TYPE" \
    --bitfield-enum "BgSize" \
    --rustified-enum "BgType" \
    --verbose \
    -- \
    --target=arm-none-eabi \
    -std=gnu17 \
    --sysroot=$WONDERFUL_ARM/arm-none-eabi \
    -isystem "\$SYSROOT/include" \
    -I$BLOCKSDS/libs/libnds/include \
    -I$WONDERFUL_ARM/arm-none-eabi/include \
    -mthumb \
    -mcpu=arm946e-s+nofp \
    -DARM9 \
> ./src/bindgen_bindings.rs