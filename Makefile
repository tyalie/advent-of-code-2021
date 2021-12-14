day=1
bin="day_$(day)"

OUT_DIR=target/bins/
TARGET_FILE=$(OUT_DIR)/$(bin).hex

.PHONY: build objcopy upload-only run

run: build objcopy upload-only

build:
	@echo "- build file for $(bin)"
	@cargo build --bin $(bin)
	@echo "=== BUILD FINISHED\n"

objcopy:
	@echo "- Generating hex file for $(bin)"
	@mkdir -p "$(OUT_DIR)"
	@cargo objcopy --bin $(bin) -- -O ihex "$(TARGET_FILE)"

upload-only:
	@echo "\n========UPLOAD========="
	@teensy_loader_cli --mcu=TEENSY41 -v -w -s "$(TARGET_FILE)"

clean:
	cargo clean
