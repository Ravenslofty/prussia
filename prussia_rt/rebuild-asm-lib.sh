#!/usr/bin/env bash

# It doesn't need to target mips64el-none-elf, as long as it's mips64el.
AS=mips64el-none-elf-as
ASFLAGS="-march=r5900 -32 -KPIC -G0 -xgot"

# Sanity check the given assembler
# Does it exist?
which $AS >/dev/null || echo "You do not have a MIPS assembler"

# Is it MIPS?
$AS --help | grep mips >/dev/null || echo "Your assembler does not support MIPS."

# Is it MIPS R5900?
$AS --help | grep r5900 >/dev/null || echo "Your assembler does not support the MIPS R5900"

mkdir -p asm-obj
rm asm-obj/*

cd src
find . -name "*.S" -exec $AS $ASFLAGS -o ../asm-obj/{}.o {} \;
cd ..

ar crs libprussia-rt.a asm-obj/*.o
