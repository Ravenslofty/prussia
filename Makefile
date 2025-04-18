
gen_svd:
	@echo Generating lib.rs
	@svd2rust --strict -i ps2.xml --target none --output-dir playstation2-pac-rs/src
	@echo Formatting
	@rustfmt playstation2-pac-rs/src/lib.rs
	@echo Done

analyse_workflows:
	zizmor -p .github/**/*