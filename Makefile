.PHONY: build, run, clean

help:
	python3 tools/build.py help

build:
	python3 tools/build.py build

run:
	python3 tools/build.py run

clean:
	rm -Rf ./output
	rm -f -Rf ./rost/target
	rm -f ./rost/Cargo.lock
	rm -f -Rf ./tools/__pycache__