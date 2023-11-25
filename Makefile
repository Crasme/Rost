.PHONY: build, run, clean

build:
	python3 tools/build.py build

run:
	python3 tools/build.py run

brun:
	make build
	make run

clean:
	rm -Rf ./output
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