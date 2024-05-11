BUILD_RELEASE=false
RELEASE=
DLWP_FEATURES=
DL_KEY=

ifeq ($(BUILD_RELEASE), true)
	RELEASE = --release
endif

all: libs
	@ DLU_KEY=$(DL_KEY) cargo build -p darklight_driver --release

libs:
	@ cargo build -p dlwp $(RELEASE)

tests:
	@ cargo test -p dlwp $(DLWP_FEATURES) -- --nocapture

docs:
	@ cargo doc
	@ mv target/doc/dlwp/ documentation/
