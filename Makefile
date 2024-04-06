BUILD_RELEASE=false
RELEASE=
DLWP_FEATURES=

ifeq ($(BUILD_RELEASE), true)
	RELEASE = --release
endif

libs:
	@ cargo build -p dlwp $(RELEASE)

tests:
	@ cargo test -p dlwp $(DLWP_FEATURES) -- --nocapture

docs:
	@ cargo doc
