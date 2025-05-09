default:
	@cargo build
	@target/debug/rtool -h
hex:
	@target/debug/rtool -t hexdump -f "assets/hello.o"

