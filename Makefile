.PHONY: build, run, clean

build:
	python3 tools/build.py

run:
	qemu-system-x86_64 -drive format=raw,file=output/rost.iso

brun:
	make build
	make run

clean:
	rm -f ./output/*
	rm -f -Rf ./rost/target
	rm -f ./rost/Cargo.lock
	rm -f -Rf ./tools/__pycache__

cbuild:
	make clean
	make build

crun:
	make clean
	make build
	make run