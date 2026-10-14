# ref-crypto: C++ oracle for the mosh crypto layer, linked against the upstream build.
# Usage: make -C ref/ref-crypto

UP := ../../upstream
# The compiler and the OpenSSL flags come from upstream's configured build
# (`$(UP)/Makefile`, written by its configure), so the oracle links the same
# way on every system upstream built on: `-lssl -lcrypto` on Linux, the
# Homebrew OpenSSL include and library paths on macOS. The defaults in the
# second argument are the Linux values, for a tree configured by hand.
upstream_var = $(or $(strip $(shell sed -n 's/^$(1) = //p' $(UP)/Makefile 2>/dev/null)),$(2))
CXX := $(call upstream_var,CXX,c++ -std=gnu++17)
CRYPTO_CFLAGS := $(call upstream_var,CRYPTO_CFLAGS,)
CRYPTO_LIBS := $(call upstream_var,CRYPTO_LIBS,-lssl -lcrypto)
CPPFLAGS := -DHAVE_CONFIG_H -I$(UP) -I$(UP)/src/include -I$(UP)/src/crypto -I$(UP)/src/util -I$(UP)/src/network -I$(UP)/src/tests $(CRYPTO_CFLAGS)
CXXFLAGS := -g -O2 -Wall
# Static archives before the libraries they need, the order GNU ld requires.
LIBS := $(UP)/src/network/libmoshnetwork.a $(UP)/src/crypto/libmoshcrypto.a $(UP)/src/util/libmoshutil.a $(CRYPTO_LIBS)

all: ref-crypto

# upstream's own RFC 7253 test program, with main renamed so ref-crypto can call it
ocb-aes.o: $(UP)/src/tests/ocb-aes.cc
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) -Dmain=ocb_aes_main -c -o $@ $<

test_utils.o: $(UP)/src/tests/test_utils.cc
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) -c -o $@ $<

ref-crypto.o: ref-crypto.cc
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) -c -o $@ $<

ref-crypto: ref-crypto.o ocb-aes.o test_utils.o $(UP)/src/network/libmoshnetwork.a $(UP)/src/crypto/libmoshcrypto.a $(UP)/src/util/libmoshutil.a
	$(CXX) $(CXXFLAGS) -o $@ ref-crypto.o ocb-aes.o test_utils.o $(LIBS)

clean:
	rm -f ref-crypto *.o

.PHONY: all clean
