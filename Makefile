# =============================================================================
.DEFAULT_GOAL:=run
.SILENT:

.PHONY: build run clean

./main: main.rs
	rustc main.rs

build: ./main

run: build
	./main

clean:
	rm main
# =============================================================================
