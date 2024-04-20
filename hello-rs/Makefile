BUILD_DIR = ../target
TARGET_PATH = ${BUILD_DIR}/ps2/debug

build_elf:
	@cargo build
	@rm ${TARGET_PATH}/hello-rs.elf
	@mv -f ${TARGET_PATH}/hello-rs ${TARGET_PATH}/hello-rs.elf
