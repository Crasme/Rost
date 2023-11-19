.PHONY: build, run

build:
	python3 tools/build.py

run:
	qemu-system-i386 output/os.bin