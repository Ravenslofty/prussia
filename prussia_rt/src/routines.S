# Bootstrap routine for PruSSia. LLVM only needs a stack pointer, and the rest we can do in Rust.
.extern _rust_start
.global _start
_start:
	j	_rust_start		# Call Rust
	lui	$sp, 0x20		# Set the stack pointer to 0x0020'0000.
