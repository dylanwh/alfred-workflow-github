BIN = alfred-workflow-github

TARGET  = $(word 2,$(subst /, ,$@))
RELEASE = $(word 3,$(subst /, ,$@))

BUILD_FLAGS = $(if $(findstring release,$@),--release,)

debug: target/universal-apple-darwin/debug/$(BIN)
clean:
	rm target/*/*/$(BIN)

target/x86_64-apple-darwin/%/$(BIN): 
	cargo build --target $(TARGET) $(BUILD_FLAGS)
	rcodesign sign \
		--smartcard-slot 9c \
	 	--smartcard-pin-env PIN \
		--code-signature-flags runtime \
		$@

target/aarch64-apple-darwin/%/$(BIN): 
	cargo build --target $(TARGET) $(BUILD_FLAGS)
	rcodesign sign \
		--smartcard-slot 9c \
	 	--smartcard-pin-env PIN \
		--code-signature-flags runtime \
		$@

target/universal-apple-darwin/%/$(BIN): target/x86_64-apple-darwin/%/$(BIN) target/aarch64-apple-darwin/%/$(BIN)
	mkdir -p $(dir $@)
	lipo -create -output $@ $^
	 rcodesign sign \
	 	--smartcard-slot 9c \
	 	--smartcard-pin-env PIN \
	 	--code-signature-flags runtime \
	 	$@

fat-binary: target/universal-apple-darwin/release/$(BIN)

build-deps:
	cargo install \
		--git https://github.com/indygreg/apple-platform-rs \
		--branch main \
		--bin rcodesign \
		--features yubikey \
		apple-codesign

gen-csr:
	rcodesign generate-certificate-signing-request \
		--smartcard-slot 9c \
		--csr-pem-file private/csr.pem
gen-key:
	rcodesign smartcard-generate-key --smartcard-slot 9c

import-key:
	rcodesign smartcard-import \
		--certificate-der-file private/developerID_application.cer \
		--existing-key \
		--smartcard-slot 9c

build/github.alfredworkflow: $(wildcard workflow/*)
	mkdir -p build
	cd workflow && zip -r ../$@ *

.PRECIOUS: target/x86_64-apple-darwin/%/$(BIN) target/aarch64-apple-darwin/%/$(BIN) target/universal-apple-darwin/%/$(BIN)
.PHONY: release debug clean build-deps gen-csr gen-key import-key fat-binary
