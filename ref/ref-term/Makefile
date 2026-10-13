# ref-term: C++ oracle linked against upstream mosh's terminal library.
# gen-wcwidth: generates crates/terminal/src/wcwidth.rs from glibc wcwidth().

UPSTREAM := ../../upstream
CXX ?= c++
CXXFLAGS := -std=gnu++17 -g -O2 -Wall -DHAVE_CONFIG_H
CPPFLAGS := -I $(UPSTREAM) -I $(UPSTREAM)/src/include
LIBS := $(UPSTREAM)/src/terminal/libmoshterminal.a $(UPSTREAM)/src/util/libmoshutil.a -lm -ltinfo

all: ref-term

ref-term: ref-term.cc $(UPSTREAM)/src/terminal/libmoshterminal.a $(UPSTREAM)/src/util/libmoshutil.a
	$(CXX) $(CXXFLAGS) $(CPPFLAGS) -o $@ ref-term.cc $(LIBS)

gen-wcwidth: gen-wcwidth.c
	$(CC) -O2 -Wall -o $@ gen-wcwidth.c

# Regenerate the Rust width table. Records the glibc version in the header.
wcwidth: gen-wcwidth
	./gen-wcwidth "$$(ldd --version | head -1)" > ../../crates/terminal/src/wcwidth.rs

clean:
	rm -f ref-term gen-wcwidth

.PHONY: all wcwidth clean
