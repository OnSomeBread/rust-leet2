all: asm
asm:
	cargo asm $(a) --intel --full-name -v --simplify --rust --reduce-labels --include-constants