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

gen-terminfo: gen-terminfo.c
	$(CC) -O2 -Wall -o $@ gen-terminfo.c -ltinfo

# Regenerate the Rust terminfo table from every entry in the local terminfo
# database (toe -a) and its aliases. Records the package versions in the header.
terminfo: gen-terminfo
	toe -a | awk '{print $$1}' | ./gen-terminfo \
	  "ncurses $$(dpkg-query -W -f='$${Version}' ncurses-base) (ncurses-base, ncurses-term), kitty-terminfo $$(dpkg-query -W -f='$${Version}' kitty-terminfo)" \
	  "$$(ldd --version | head -1)" > ../../crates/terminal/src/terminfo.rs
	rustfmt --edition 2021 ../../crates/terminal/src/terminfo.rs

clean:
	rm -f ref-term gen-wcwidth gen-terminfo

.PHONY: all wcwidth terminfo clean
