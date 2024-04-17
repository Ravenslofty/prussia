# Bootstrap routine for PruSSia. LLVM only needs a stack pointer, and the rest we can do in Rust.
.extern _rust_start
.global _start
_start:
	j	_rust_start		# Call Rust
	lui	$sp, 0x20		# Set the stack pointer to 0x0020'0000.

# Read coprocessor 0 Status register. This *could* be done with inline assembly, but I'm going to
# aim for compilation on stable.
.global _read_status
_read_status:
	jr	$ra			# Return to Rust...
	mfc0	$2, $12			# with the Status register value.

# Write coprocessor 0 Status register.
# $4 (a0) = value.
.global _write_status
_write_status:
	jr	$ra			# Return to Rust...
	mtc0	$4, $12			# after setting the Status register.

# Read coprocessor 0 BadVAddr register.
.global _read_badvaddr
_read_badvaddr:
	mfc0	$2, $8
	jr $ra

# Read coprocessor 0 Count register.
.global _read_timercount
_read_timercount:
	mfc0	$2, $9
	jr $ra

# Write coprocessor 0 Count register.
.global _write_timercount
_write_timercount:
	mtc0	$4, $9
	jr	$ra

# Read coprocessor 0 Compare register.
.global _read_compare
_read_compare:
	mfc0	$2, $11
	jr $ra

# Write coprocessor 0 Compare register.
.global _write_compare
_write_compare:
	mtc0	$4, $11
	jr	$ra

# Read coprocessor 0 EPC register.
.global _read_epc
_read_epc:
	mfc0	$2, $14
	jr $ra

# Write coprocessor 0 EPC register.
.global _write_epc
_write_epc:
	mtc0	$4, $14
	jr	$ra

# Read a u64 atomically. Rust would make two reads, but this would just be the truncated bottom
# half. This method is always correct though.
# $4 (a0) = address.
.global _read_u64
_read_u64:
	nor	$8, $0, $0		# Create a 64-bit all-ones mask in $t0.
	dsrl32	$8, 0			# Turn it into 32-bit.
	ld	$2, 0($4)		# Load from memory into $v0.
	move	$3, $2			# Copy into $v1.
	and	$2, $8			# Return lowest word in $v0.
	dsrl32	$3, 0			# Return highest word in $v1.
	jr	$ra			# And return to caller.
	nop

# Write a u64 atomically. Rust would make two writes, but this would probably confuse the device
# we are writing to. This method is always correct though.
# $4 (a0) = address.
# $5 (a1) = value (lower 32 bits).
# $6 (a2) = value (upper 32 bits).
.global _write_u64
_write_u64:
	dsll32	$6, 0			# upper <<= 32
	or	$5, $6			# lower |= upper
	jr	$ra
	sd	$6, 0($4)		# *address = lower
