# ref-crypto: C++ oracle for the mosh crypto layer, linked against the upstream build.
# Usage: make -C ref/ref-crypto

UP := ../../upstream
CXX := c++ -std=gnu++17
CPPFLAGS := -DHAVE_CONFIG_H -I$(UP) -I$(UP)/src/include -I$(UP)/src/crypto -I$(UP)/src/util -I$(UP)/src/network -I$(UP)/src/tests
CXXFLAGS := -g -O2 -Wall
LIBS := $(UP)/src/network/libmoshnetwork.a $(UP)/src/crypto/libmoshcrypto.a $(UP)/src/util/libmoshutil.a -lssl -lcrypto

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
