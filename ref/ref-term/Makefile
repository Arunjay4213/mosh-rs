# ref-term: C++ oracle linked against upstream mosh's terminal library.
# gen-wcwidth: generates crates/terminal/src/wcwidth.rs from glibc wcwidth().

UPSTREAM := ../../upstream
# The compiler and the terminfo library come from upstream's configured
# build (`$(UPSTREAM)/Makefile`, written by its configure), so the oracle
# links the same way on every system upstream built on: `-ltinfo` on Linux,
# `-lncurses` on macOS, which has no separate tinfo library. The defaults
# in the second argument are the Linux values, for a tree configured by hand.
upstream_var = $(or $(strip $(shell sed -n 's/^$(1) = //p' $(UPSTREAM)/Makefile 2>/dev/null)),$(2))
CXX := $(call upstream_var,CXX,c++ -std=gnu++17)
TINFO_LIBS := $(call upstream_var,TINFO_LIBS,-ltinfo)
CXXFLAGS := -g -O2 -Wall -DHAVE_CONFIG_H
CPPFLAGS := -I $(UPSTREAM) -I $(UPSTREAM)/src/include
# Static archives before the libraries they need, the order GNU ld requires.
LIBS := $(UPSTREAM)/src/terminal/libmoshterminal.a $(UPSTREAM)/src/util/libmoshutil.a -lm $(TINFO_LIBS)

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
