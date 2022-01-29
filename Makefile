
# ---------
# デバッグ
# ---------
.PHONY: debug
debug: ## debug
	cd ./sample/ && cargo watch -x run
