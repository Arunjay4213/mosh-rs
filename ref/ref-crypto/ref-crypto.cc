/*
    Mosh: the mobile shell
    Copyright 2012 Keith Winstein

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.

    In addition, as a special exception, the copyright holders give
    permission to link the code of portions of this program with the
    OpenSSL library under certain conditions as described in each
    individual source file, and distribute linked combinations including
    the two.

    You must obey the GNU General Public License in all respects for all
    of the code used other than OpenSSL. If you modify file(s) with this
    exception, you may extend this exception to your version of the
    file(s), but you are not obligated to do so. If you do not wish to do
    so, delete this exception statement from your version. If you delete
    this exception statement from all source files in the program, then
    also delete it here.
*/

/* ref-crypto: oracle for the mosh crypto layer. Uses upstream
   Crypto::Session, Crypto::Base64Key, Crypto::Nonce and Network::Packet.
   CLI is documented in README.md in this directory. */

#include <cerrno>
#include <cinttypes>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <iterator>
#include <string>
#include <sys/wait.h>
#include <unistd.h>

#include "src/crypto/crypto.h"
#include "src/network/network.h"

int ocb_aes_main( int argc, char* argv[] );

using namespace Crypto;
using namespace Network;

static int usage( void )
{
  fprintf( stderr, "usage: ref-crypto enc KEY DIR SEQ TS TSR < payload\n"
                   "       ref-crypto dec KEY < hex\n"
                   "       ref-crypto vectors\n" );
  return 2;
}

static std::string read_stdin( void )
{
  std::string s( ( std::istreambuf_iterator<char>( std::cin ) ), std::istreambuf_iterator<char>() );
  return s;
}

static std::string to_hex( const std::string& s )
{
  static const char digits[] = "0123456789abcdef";
  std::string ret;
  for ( unsigned char c : s ) {
    ret += digits[c >> 4];
    ret += digits[c & 0xf];
  }
  return ret;
}

static int hexval( char c )
{
  if ( c >= '0' && c <= '9' )
    return c - '0';
  if ( c >= 'a' && c <= 'f' )
    return c - 'a' + 10;
  if ( c >= 'A' && c <= 'F' )
    return c - 'A' + 10;
  return -1;
}

/* Returns false on malformed hex. ASCII whitespace is ignored. */
static bool from_hex( const std::string& in, std::string& out )
{
  std::string digits;
  for ( char c : in ) {
    if ( c == ' ' || c == '\n' || c == '\r' || c == '\t' )
      continue;
    if ( hexval( c ) < 0 )
      return false;
    digits += c;
  }
  if ( digits.size() % 2 )
    return false;
  out.clear();
  for ( size_t i = 0; i < digits.size(); i += 2 ) {
    out += (char)( hexval( digits[i] ) * 16 + hexval( digits[i + 1] ) );
  }
  return true;
}

static bool parse_u64( const char* s, uint64_t max, uint64_t& out )
{
  if ( *s < '0' || *s > '9' )
    return false;
  char* end;
  errno = 0;
  unsigned long long v = strtoull( s, &end, 10 );
  if ( errno != 0 || *end != '\0' || v > max )
    return false;
  out = v;
  return true;
}

static int do_enc( int argc, char* argv[] )
{
  if ( argc != 7 )
    return usage();
  uint64_t dir, seq, ts, tsr;
  if ( !parse_u64( argv[3], 1, dir ) || !parse_u64( argv[4], UINT64_MAX, seq )
       || !parse_u64( argv[5], 65535, ts ) || !parse_u64( argv[6], 65535, tsr ) ) {
    return usage();
  }
  std::string payload = read_stdin();
  try {
    Base64Key key( argv[2] );
    Session session( key );
    /* Packet's public constructor draws seq from Crypto::unique(), so build the
       plaintext the way Packet::toMessage lays it out, parse it with upstream
       Packet( const Message& ), and re-serialize with upstream toMessage(). */
    uint64_t direction_seq = ( dir << 63 ) | ( seq & ~( uint64_t( 1 ) << 63 ) );
    std::string text;
    text += (char)( ts >> 8 );
    text += (char)( ts & 0xff );
    text += (char)( tsr >> 8 );
    text += (char)( tsr & 0xff );
    text += payload;
    Packet p( Message( Nonce( direction_seq ), text ) );
    if ( p.seq != ( seq & ~( uint64_t( 1 ) << 63 ) ) || p.direction != ( dir ? TO_CLIENT : TO_SERVER )
         || p.timestamp != ts || p.timestamp_reply != tsr || p.payload != payload ) {
      fprintf( stderr, "ref-crypto: Packet round trip mismatch\n" );
      return 3;
    }
    std::string wire = session.encrypt( p.toMessage() );
    printf( "%s\n", to_hex( wire ).c_str() );
  } catch ( const CryptoException& e ) {
    fprintf( stderr, "ref-crypto: %s\n", e.what() );
    printf( "error\n" );
    return 1;
  }
  return 0;
}

static int do_dec( int argc, char* argv[] )
{
  if ( argc != 3 )
    return usage();
  std::string wire;
  if ( !from_hex( read_stdin(), wire ) ) {
    fprintf( stderr, "ref-crypto: malformed hex on stdin\n" );
    return 2;
  }
  try {
    Base64Key key( argv[2] );
    Session session( key );
    Packet p( session.decrypt( wire ) );
    printf( "dir=%d seq=%" PRIu64 " ts=%u tsr=%u payload=%s\n",
            p.direction == TO_CLIENT ? 1 : 0,
            p.seq,
            (unsigned)p.timestamp,
            (unsigned)p.timestamp_reply,
            to_hex( p.payload ).c_str() );
  } catch ( const CryptoException& e ) {
    fprintf( stderr, "ref-crypto: %s\n", e.what() );
    printf( "error\n" );
    return 1;
  }
  return 0;
}

static int do_vectors( void )
{
  /* upstream ocb-aes.cc reports failure with abort() via fatal_assert,
     so run it in a child and look at how it exited. */
  fflush( stdout );
  pid_t pid = fork();
  if ( pid < 0 ) {
    perror( "fork" );
    return 2;
  }
  if ( pid == 0 ) {
    char name[] = "ocb-aes";
    char* args[] = { name, NULL };
    _exit( ocb_aes_main( 1, args ) );
  }
  int status;
  if ( waitpid( pid, &status, 0 ) != pid ) {
    perror( "waitpid" );
    return 2;
  }
  if ( WIFEXITED( status ) && WEXITSTATUS( status ) == 0 ) {
    printf( "ok\n" );
    return 0;
  }
  printf( "FAIL\n" );
  return 1;
}

int main( int argc, char* argv[] )
{
  if ( argc < 2 )
    return usage();
  if ( !strcmp( argv[1], "enc" ) )
    return do_enc( argc, argv );
  if ( !strcmp( argv[1], "dec" ) )
    return do_dec( argc, argv );
  if ( !strcmp( argv[1], "vectors" ) && argc == 2 )
    return do_vectors();
  return usage();
}
