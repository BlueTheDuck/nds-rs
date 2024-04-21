clang_version="11.1.0"

echo '#![allow(warnings)]' > src/bindings.rs

WONDERFUL_ARM="$WONDERFUL_TOOLCHAIN/toolchain/gcc-arm-none-eabi/"
BLOCKSDS="$WONDERFUL_TOOLCHAIN/thirdparty/blocksds/core/"
NDS_H="$BLOCKSDS/libs/libnds/include/nds.h"

bindgen \
    "$NDS_H" \
    --rust-target nightly \
    --use-core \
    --distrust-clang-mangling \
    --no-doc-comments \
    --no-layout-tests \
    --ctypes-prefix "core::ffi" \
    --no-prepend-enum-name \
    --generate "functions,types,vars" \
    --blocklist-type "u(8|16|32|64)" \
    --blocklist-type "__builtin_va_list" \
    --blocklist-type "__va_list" \
    -- \
    --target=arm-none-eabi \
    --sysroot=$WONDERFUL_ARM/arm-none-eabi \
    -isystem$WONDERFUL_ARM/arm-none-eabi/include \
    -isystem/usr/lib/clang/$clang_version/include \
    -I$BLOCKSDS/libs/libnds/include \
    -mfloat-abi=soft \
    -march=armv5te \
    -mtune=arm946e-s \
    -DARM9 \
>> src/bindings.rs