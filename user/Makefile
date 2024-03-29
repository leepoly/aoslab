.PHONY: build, unzip, build-lfs

TARGET      := riscv64imac-unknown-none-elf
MODE        := debug

# 用户程序目录
SRC_DIR		:= src/bin
# 编译后执行文件目录
TARGET_DIR	:= target/$(TARGET)/$(MODE)
# 用户程序源文件
SRC_FILES	:= $(wildcard $(SRC_DIR)/*.rs)
# 根据源文件取得编译后的执行文件
BIN_FILES	:= $(patsubst $(SRC_DIR)/%.rs, $(TARGET_DIR)/%, $(SRC_FILES))

OUT_DIR		:= build/disk
OUT_DIR2	:= build/disk2
IMG_FILE	:= build/raw.img
QCOW_FILE	:= build/disk.img

# 安装 rcore-fs-fuse 工具
dependency:
ifeq ($(shell which rcore-fs-fuse),)
	@echo Installing rcore-fs-fuse
	@cargo install rcore-fs-fuse --git https://github.com/rcore-os/rcore-fs
endif

# 编译、打包、格式转换、预留空间

build-sfs: dependency
	@cargo build
	@make build -C ../../rcore-fs/
	@echo Targets: $(patsubst $(SRC_DIR)/%.rs, %, $(SRC_FILES))
	@rm -rf $(OUT_DIR)
	@mkdir -p $(OUT_DIR)
	@cp $(BIN_FILES) $(OUT_DIR)
	@mkdir -p $(OUT_DIR)/test
	@dd if=/dev/zero of=$(OUT_DIR)/test/temp123 bs=1k count=2
	echo Start making sfs...
	@./rcore-lfs-fuse --fs sfs $(IMG_FILE) $(OUT_DIR) zip
	@qemu-img convert -f raw $(IMG_FILE) -O qcow2 $(QCOW_FILE)
	@qemu-img resize $(QCOW_FILE) +1G

random-test: dependency
	@make build -C ../../rcore-fs/
	@rm -rf $(OUT_DIR)
	@python3 ../../rcore-fs/rcore-fs-lfs/src/trace/file_generator.py $(OUT_DIR)
	echo Start making lfs...
	@./rcore-lfs-fuse --fs lfs $(IMG_FILE) $(OUT_DIR) zip
	@qemu-img convert -f raw $(IMG_FILE) -O qcow2 $(QCOW_FILE)
	@qemu-img resize $(QCOW_FILE) +1G

test: dependency
	@make build -C ../../rcore-fs/
	# @rm -rf $(OUT_DIR)
	# @python3 ../../rcore-fs/rcore-fs-lfs/src/trace/file_generator.py $(OUT_DIR)
	# echo Start making lfs...
	# @./rcore-lfs-fuse --fs lfs $(IMG_FILE) $(OUT_DIR) zip
	@./rcore-lfs-fuse --fs lfs $(IMG_FILE) $(OUT_DIR) test
	@qemu-img convert -f raw $(IMG_FILE) -O qcow2 $(QCOW_FILE)
	@qemu-img resize $(QCOW_FILE) +1G

random-test-sfs: dependency
	@make build -C ../../rcore-fs/
	# @python3 ../../rcore-fs/rcore-fs-lfs/src/trace/file_generator.py $(OUT_DIR)-sfs
	echo Start making sfs...
	@./rcore-lfs-fuse --fs sfs $(IMG_FILE)-sfs $(OUT_DIR) zip
	@qemu-img convert -f raw $(IMG_FILE)-sfs -O qcow2 $(QCOW_FILE)-sfs
	@qemu-img resize $(QCOW_FILE)-sfs +1G

build: dependency
	@cargo build
	@make build -C ../../rcore-fs/
	@echo Targets: $(patsubst $(SRC_DIR)/%.rs, %, $(SRC_FILES))
	@rm -rf $(OUT_DIR)
	@mkdir -p $(OUT_DIR)
	@cp $(BIN_FILES) $(OUT_DIR)
	@mkdir -p $(OUT_DIR)/test
	@dd if=/dev/zero of=$(OUT_DIR)/temp111 bs=1k count=2
	echo Start making lfs...
	@./rcore-lfs-fuse --fs lfs $(IMG_FILE) $(OUT_DIR) zip
	@qemu-img convert -f raw $(IMG_FILE) -O qcow2 $(QCOW_FILE)
	@qemu-img resize $(QCOW_FILE) +1G

unzip: dependency
	@make build -C ../../rcore-fs/
	@rm -rf $(OUT_DIR2)
	@./rcore-lfs-fuse --fs lfs $(IMG_FILE) $(OUT_DIR2) unzip

unzip-sfs: dependency
	@make build -C ../../rcore-fs/
	@rm -rf $(OUT_DIR)-sfs
	@./rcore-lfs-fuse --fs sfs $(IMG_FILE)-sfs $(OUT_DIR)-sfs unzip

clean:
	@cargo clean
	@rm -rf $(OUT_DIR) $(IMG_FILE) $(QCOW_FILE)