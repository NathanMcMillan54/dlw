BUILD_RELEASE=false
RELEASE=
DLWP_FEATURES=
DL_KEY=

ifeq ($(BUILD_RELEASE), true)
	RELEASE = --release
endif

all: libs driver
	@ cargo build -p dlcmd --release
	@ cargo build -p new_dlukey --release
	@ cargo build -p dlup --release

libs: tests
	@ cargo build -p dlwp $(RELEASE)

tests:
	@ cargo test -p dlwp $(DLWP_FEATURES) -- --nocapture

build_tools:
	@ cargo build -p dlcmd --release
	@ cargo build -p new_dlukey --release
	@ cargo build -p dlup --release
	@ cargo build -p client --release

move_tools:
	sudo mv target/release/new_dlukey /sbin/new_dlukey
	sudo mv target/release/dlcmd /sbin/dlcmd
	sudo mv target/release/dlup /sbin/dlup
	sudo mv target/release/client /sbin/dl_client

driver:
	@ DLU_KEY=$(DL_KEY) cargo build -p darklight_driver --release

docs:
	@ cargo doc
