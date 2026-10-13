/*
 * gen-terminfo.c: generate crates/terminal/src/terminfo.rs, the terminfo
 * answers upstream mosh's Display(true) would get for every terminal type.
 *
 * Upstream terminaldisplayinit.cc calls setupterm() for $TERM and reads
 * tigetstr("ech") != NULL, tigetflag("bce"), tigetstr("smcup") and
 * tigetstr("rmcup"). Title support is decided from the TERM string alone, not
 * from terminfo, so it is not in the table. PLAN.md section 9.6.
 *
 * Input on stdin: entry names, one per line (`toe -a | awk '{print $1}'`).
 * Every alias in each entry's names line (ncurses `ttytype`, e.g.
 * "xterm-kitty|KovIdTTY") is added, and every name, alias or not, is looked up
 * with its own setupterm() call, exactly as upstream would with TERM=<name>.
 *
 * Usage: gen-terminfo "<header line>"... < names > terminfo.rs
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 */
#include <curses.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <term.h>

#define MAXNAMES 8192
#define MAXCAPS 2048

struct caps
{
  int ech, bce;
  const char *smcup, *rmcup; /* NULL = absent */
};

static char* names[MAXNAMES];
static int nnames;
static struct caps table[MAXCAPS];
static int ncaps;
static int name_cap[MAXNAMES];

static void add_name( const char* n )
{
  for ( int i = 0; i < nnames; i++ ) {
    if ( 0 == strcmp( names[i], n ) ) {
      return;
    }
  }
  if ( nnames == MAXNAMES ) {
    fprintf( stderr, "too many names\n" );
    exit( 1 );
  }
  names[nnames++] = strdup( n );
}

static int cmp_names( const void* a, const void* b )
{
  return strcmp( *(char* const*)a, *(char* const*)b );
}

static int same_str( const char* a, const char* b )
{
  if ( !a || !b ) {
    return a == b;
  }
  return 0 == strcmp( a, b );
}

/* Rust byte-string literal for a terminfo string (it may hold any byte). */
static void print_str( const char* s )
{
  if ( !s ) {
    printf( "None" );
    return;
  }
  printf( "Some(b\"" );
  for ( const unsigned char* p = (const unsigned char*)s; *p; p++ ) {
    if ( *p == '"' || *p == '\\' ) {
      printf( "\\%c", *p );
    } else if ( *p < 0x20 || *p >= 0x7f ) {
      printf( "\\x%02x", *p );
    } else {
      putchar( *p );
    }
  }
  printf( "\")" );
}

static int has_high_byte( const char* s )
{
  for ( const unsigned char* p = (const unsigned char*)s; s && *p; p++ ) {
    if ( *p >= 0x80 ) {
      return 1;
    }
  }
  return 0;
}

/* What upstream's Display(true) reads, for TERM=name. Returns -1 if
   setupterm fails (upstream would throw). */
static int lookup( const char* name, struct caps* out )
{
  int errret = -2;
  if ( setupterm( (char*)name, 1, &errret ) != OK ) {
    return -1;
  }
  const char* ech = tigetstr( (char*)"ech" );
  int bce = tigetflag( (char*)"bce" );
  const char* smcup = tigetstr( (char*)"smcup" );
  const char* rmcup = tigetstr( (char*)"rmcup" );
  if ( ech == (const char*)-1 || bce == -1 || smcup == (const char*)-1 || rmcup == (const char*)-1 ) {
    fprintf( stderr, "invalid capability name\n" );
    exit( 1 );
  }
  out->ech = ech != NULL;
  out->bce = bce;
  out->smcup = smcup ? strdup( smcup ) : NULL;
  out->rmcup = rmcup ? strdup( rmcup ) : NULL;
  del_curterm( cur_term );
  return 0;
}

int main( int argc, char** argv )
{
  char line[4096];
  while ( fgets( line, sizeof line, stdin ) ) {
    line[strcspn( line, "\r\n" )] = 0;
    if ( line[0] ) {
      add_name( line );
    }
  }
  /* Add aliases from each entry's names line: every '|' field but the last
     (the description), when there is more than one field. */
  int primaries = nnames;
  for ( int i = 0; i < primaries; i++ ) {
    int errret = -2;
    if ( setupterm( names[i], 1, &errret ) != OK ) {
      continue;
    }
    char buf[4096];
    snprintf( buf, sizeof buf, "%s", ttytype );
    del_curterm( cur_term );
    char* fields[64];
    int nf = 0;
    for ( char* tok = strtok( buf, "|" ); tok && nf < 64; tok = strtok( NULL, "|" ) ) {
      fields[nf++] = tok;
    }
    for ( int k = 0; k + 1 < nf; k++ ) {
      add_name( fields[k] );
    }
  }
  qsort( names, nnames, sizeof names[0], cmp_names );

  int kept = 0;
  for ( int i = 0; i < nnames; i++ ) {
    struct caps c;
    if ( lookup( names[i], &c ) != 0 ) {
      /* hardcopy or broken entries: upstream throws; the port falls back to xterm */
      fprintf( stderr, "skipping %s: setupterm failed\n", names[i] );
      continue;
    }
    if ( has_high_byte( c.smcup ) || has_high_byte( c.rmcup ) ) {
      fprintf( stderr, "note: %s has bytes >= 0x80 in smcup/rmcup\n", names[i] );
    }
    int j;
    for ( j = 0; j < ncaps; j++ ) {
      if ( table[j].ech == c.ech && table[j].bce == c.bce && same_str( table[j].smcup, c.smcup )
           && same_str( table[j].rmcup, c.rmcup ) ) {
        break;
      }
    }
    if ( j == ncaps ) {
      if ( ncaps == MAXCAPS ) {
        fprintf( stderr, "too many tuples\n" );
        exit( 1 );
      }
      table[ncaps++] = c;
    }
    names[kept] = names[i];
    name_cap[kept] = j;
    kept++;
  }

  printf( "// Mosh: the mobile shell\n"
          "// Copyright 2012 Keith Winstein\n"
          "//\n"
          "// This program is free software: you can redistribute it and/or modify\n"
          "// it under the terms of the GNU General Public License as published by\n"
          "// the Free Software Foundation, either version 3 of the License, or\n"
          "// (at your option) any later version.\n"
          "//\n"
          "// This program is distributed in the hope that it will be useful,\n"
          "// but WITHOUT ANY WARRANTY; without even the implied warranty of\n"
          "// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\n"
          "// GNU General Public License for more details.\n"
          "//\n"
          "// You should have received a copy of the GNU General Public License\n"
          "// along with this program.  If not, see <http://www.gnu.org/licenses/>.\n"
          "//\n"
          "// Ported from: the terminfo queries in upstream/src/terminal/terminaldisplayinit.cc\n"
          "//\n"
          "// GENERATED by ref/ref-term/gen-terminfo.c (make -C ref/ref-term terminfo). Do not edit.\n" );
  for ( int i = 1; i < argc; i++ ) {
    printf( "// %s\n", argv[i] );
  }
  printf( "// %d names (entries from `toe -a` and their aliases), %d distinct capability tuples.\n"
          "\n"
          "//! The terminfo answers upstream's `Display(true)` gets for every terminal\n"
          "//! type in the devbox terminfo database (PLAN.md section 9.6).\n"
          "\n"
          "/// What upstream reads from terminfo for one terminal type.\n"
          "#[derive(Clone, Copy, Debug, PartialEq, Eq)]\n"
          "pub struct TermCaps {\n"
          "    /// `tigetstr(\"ech\") != NULL`\n"
          "    pub ech: bool,\n"
          "    /// `tigetflag(\"bce\")`\n"
          "    pub bce: bool,\n"
          "    /// `tigetstr(\"smcup\")` bytes, None when absent\n"
          "    pub smcup: Option<&'static [u8]>,\n"
          "    /// `tigetstr(\"rmcup\")` bytes, None when absent\n"
          "    pub rmcup: Option<&'static [u8]>,\n"
          "}\n"
          "\n"
          "/// The distinct capability tuples.\n"
          "pub static CAPS: &[TermCaps] = &[\n",
          kept,
          ncaps );
  for ( int j = 0; j < ncaps; j++ ) {
    printf( "    TermCaps {\n        ech: %s,\n        bce: %s,\n        smcup: ",
            table[j].ech ? "true" : "false",
            table[j].bce ? "true" : "false" );
    print_str( table[j].smcup );
    printf( ",\n        rmcup: " );
    print_str( table[j].rmcup );
    printf( ",\n    },\n" );
  }
  printf( "];\n\n/// Terminal type name to index into `CAPS`, sorted by name (byte order).\n"
          "pub static NAMES: &[(&str, u16)] = &[\n" );
  for ( int i = 0; i < kept; i++ ) {
    printf( "    (\"%s\", %d),\n", names[i], name_cap[i] );
  }
  printf( "];\n\n"
          "/// The capabilities for terminal type `name`, if the database has it.\n"
          "pub fn lookup(name: &str) -> Option<&'static TermCaps> {\n"
          "    NAMES\n"
          "        .binary_search_by(|(n, _)| (*n).cmp(name))\n"
          "        .ok()\n"
          "        .map(|i| &CAPS[NAMES[i].1 as usize])\n"
          "}\n" );
  fprintf( stderr, "%d names, %d tuples\n", kept, ncaps );
  return 0;
}
