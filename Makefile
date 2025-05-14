
gen_svd:
	@echo Generating lib.rs
	@rm -rf playstation2-pac-rs/src/
	@mkdir playstation2-pac-rs/src/
	@svd2rust --strict -i ps2.xml --target none --output-dir playstation2-pac-rs/src
	@echo Formatting
	@form -i playstation2-pac-rs/src/lib.rs -o playstation2-pac-rs/src/
	@cargo fmt -p playstation2
	@echo Done

analyse_workflows:
	zizmor -p .github/**/*

install_dev_cargo_subcommands:
	cargo install --locked \
		zizmor \
		svd2rust \
		form \
		cargo-audit