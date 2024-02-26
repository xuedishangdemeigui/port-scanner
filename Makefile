BINARY_NAME = "port-scanner"

build:
	sh build.sh

clean:
	rm -rf ./output
	rm ${BINARY_NAME}

