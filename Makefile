#!/usr/bin/make -f

EXEC_NAME:=jinjer
RUST_RELEASE_TARGET:=x86_64-unknown-linux-musl

release:
	@cargo build --release --target $(RUST_RELEASE_TARGET)

strip:
	@strip target/$(RUST_RELEASE_TARGET)/release/$(EXEC_NAME)

deploy: release strip
ifndef DEPLOY_HOST
$(error DEPLOY_HOST is not set.)
endif

	@echo "Deploying binary to $(DEPLOY_HOST)..."
	@scp -q -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null \
		target/$(RUST_RELEASE_TARGET)/release/$(EXEC_NAME) \
		$(DEPLOY_HOST):bin/

shell:
ifndef DEPLOY_HOST
$(error DEPLOY_HOST is not set.)
endif
	@ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null \
		$(DEPLOY_HOST)