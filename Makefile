.PHONY: build, run

build:
	python3 tools/build.py

run:
	qemu-system-i386 os.hex