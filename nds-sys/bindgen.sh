clang_version="11.1.0"

echo '#![allow(warnings)]' > src/bindings.rs

bindgen "$DEVKITPRO/libnds/include/nds.h" \
    --rust-target nightly \
    --use-core \
    --distrust-clang-mangling \
    --no-doc-comments \
    --no-layout-tests \
    --ctypes-prefix "::libc" \
    --no-prepend-enum-name \
    --generate "functions,types,vars" \
    --blacklist-type "u(8|16|32|64)" \
    --blacklist-type "__builtin_va_list" \
    --blacklist-type "__va_list" \
    -- \
    --target=arm-none-eabi \
    --sysroot=$DEVKITARM/arm-none-eabi \
    -isystem$DEVKITARM/arm-none-eabi/include \
    -isystem/usr/lib/clang/$clang_version/include \
    -I$DEVKITPRO/libnds/include \
    -mfloat-abi=soft \
    -march=armv5te \
    -mtune=arm946e-s \
    -DARM9 \
>> src/bindings.rs