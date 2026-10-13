/*
    ref-term: differential-testing oracle for the mosh-rs terminal port.

    Feeds bytes to upstream mosh's own Terminal::Emulator (linked from
    upstream/src/terminal/libmoshterminal.a) exactly the way
    Terminal::Complete::act does, and prints a deterministic text dump of
    the framebuffer, or the bytes of Display::new_frame. The Rust twin is
    tests/src/bin/vt-check.rs; both must print identical bytes.
    See README.md for the CLI and the dump format.

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.
*/

/* Standard headers first, so the access hack below only affects mosh's. */
#include <algorithm>
#include <cassert>
#include <cerrno>
#include <climits>
#include <clocale>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <cwchar>
#include <deque>
#include <fstream>
#include <iostream>
#include <iterator>
#include <list>
#include <map>
#include <memory>
#include <sstream>
#include <string>
#include <typeinfo>
#include <utility>
#include <vector>

/* The dump must show private state (renditions, tabs, saved cursor...).
   Same object layout; only access checking changes. */
#define private public
#define protected public
#include "src/terminal/parser.h"
#include "src/terminal/terminal.h"
#include "src/terminal/terminaldisplay.h"
#include "src/terminal/terminalframebuffer.h"
#undef private

/* From ncurses <term.h>, which cannot be included here: its capability
   macros clash with mosh's names (upstream keeps its terminfo calls in
   terminaldisplayinit.cc for the same reason). */
extern "C" int tigetnum( const char* capname );
#undef protected

using namespace Terminal;

static std::string hex( const std::string& s )
{
  static const char digits[] = "0123456789abcdef";
  std::string out;
  for ( unsigned char c : s ) {
    out.push_back( digits[c >> 4] );
    out.push_back( digits[c & 15] );
  }
  return out;
}

static std::string wlist( const std::vector<wchar_t>& v )
{
  std::string out;
  char buf[16];
  for ( size_t i = 0; i < v.size(); i++ ) {
    snprintf( buf, sizeof buf, "%s%x", i ? "," : "", (unsigned int)v[i] );
    out.append( buf );
  }
  return out;
}

static std::string link( const Hyperlink& h )
{
  if ( !h.rep ) {
    return "none";
  }
  return hex( h.rep->params ) + "|" + hex( h.rep->url );
}

static std::string rend( const Renditions& r )
{
  char buf[96];
  snprintf( buf,
            sizeof buf,
            "%u %u %u",
            (unsigned int)r.attributes,
            (unsigned int)r.foreground_color,
            (unsigned int)r.background_color );
  return buf;
}

static void dump( const Framebuffer& fb, const std::string& to_host, std::string& o )
{
  const DrawState& ds = fb.ds;
  char buf[256];
  snprintf( buf, sizeof buf, "size %d %d\n", ds.width, ds.height );
  o += buf;
  snprintf( buf,
            sizeof buf,
            "cursor %d %d visible %d next_print_will_wrap %d\n",
            ds.cursor_row,
            ds.cursor_col,
            (int)ds.cursor_visible,
            (int)ds.next_print_will_wrap );
  o += buf;
  snprintf( buf, sizeof buf, "combining %d %d\n", ds.combining_char_row, ds.combining_char_col );
  o += buf;
  snprintf( buf,
            sizeof buf,
            "origin %d autowrap %d insert %d cursor_keys_app %d reverse_video %d bracketed_paste %d\n",
            (int)ds.origin_mode,
            (int)ds.auto_wrap_mode,
            (int)ds.insert_mode,
            (int)ds.application_mode_cursor_keys,
            (int)ds.reverse_video,
            (int)ds.bracketed_paste );
  o += buf;
  snprintf( buf,
            sizeof buf,
            "mouse mode %d focus %d altscroll %d encoding %d\n",
            (int)ds.mouse_reporting_mode,
            (int)ds.mouse_focus_event,
            (int)ds.mouse_alternate_scroll,
            (int)ds.mouse_encoding_mode );
  o += buf;
  snprintf(
    buf, sizeof buf, "scroll_region %d %d\n", ds.scrolling_region_top_row, ds.scrolling_region_bottom_row );
  o += buf;
  o += "default_tabs ";
  o += ds.default_tabs ? "1" : "0";
  o += " tabs ";
  bool first = true;
  for ( size_t i = 0; i < ds.tabs.size(); i++ ) {
    if ( ds.tabs[i] ) {
      snprintf( buf, sizeof buf, "%s%zu", first ? "" : ",", i );
      o += buf;
      first = false;
    }
  }
  o += "\n";
  o += "renditions " + rend( ds.renditions ) + "\n";
  o += "hyperlink " + link( ds.hyperlink ) + "\n";
  snprintf( buf,
            sizeof buf,
            "saved %d %d autowrap %d origin %d renditions ",
            ds.save.cursor_row,
            ds.save.cursor_col,
            (int)ds.save.auto_wrap_mode,
            (int)ds.save.origin_mode );
  o += buf;
  o += rend( ds.save.renditions ) + "\n";
  snprintf( buf, sizeof buf, "title_initialized %d\n", (int)fb.title_initialized );
  o += buf;
  o += "icon " + wlist( fb.icon_name ) + "\n";
  o += "title " + wlist( fb.window_title ) + "\n";
  o += "clipboard " + wlist( fb.clipboard ) + "\n";
  snprintf( buf, sizeof buf, "bell_count %u\n", fb.bell_count );
  o += buf;
  o += "to_host " + hex( to_host ) + "\n";
  for ( size_t r = 0; r < fb.rows.size(); r++ ) {
    const Row& row = *fb.rows[r];
    snprintf( buf,
              sizeof buf,
              "row %zu gen %llu wrap %d\n",
              r,
              (unsigned long long)row.gen,
              (int)row.get_wrap() );
    o += buf;
    for ( size_t c = 0; c < row.cells.size(); c++ ) {
      const Cell& cell = row.cells[c];
      snprintf( buf, sizeof buf, "  cell %zu contents=", c );
      o += buf;
      o += hex( cell.contents );
      snprintf( buf,
                sizeof buf,
                " wide %d fallback %d wrap %d renditions ",
                (int)cell.wide,
                (int)cell.fallback,
                (int)cell.wrap );
      o += buf;
      o += rend( cell.renditions );
      o += " link=" + link( cell.hyperlink ) + "\n";
    }
  }
}

/* Feed bytes exactly as Terminal::Complete::act does. */
static void feed( Parser::UTF8Parser& parser, Emulator& emu, const std::string& str, std::string& to_host )
{
  Parser::Actions actions;
  for ( unsigned int i = 0; i < str.size(); i++ ) {
    parser.input( str[i], actions );
    for ( Parser::Actions::iterator it = actions.begin(); it != actions.end(); it++ ) {
      Parser::Action& act = **it;
      act.act_on_terminal( &emu );
    }
    actions.clear();
  }
  to_host += emu.read_octets_to_host();
}

static std::string read_all( std::istream& in )
{
  return std::string( std::istreambuf_iterator<char>( in ), std::istreambuf_iterator<char>() );
}

static std::string read_file( const char* path )
{
  std::ifstream f( path, std::ios::binary );
  if ( !f ) {
    fprintf( stderr, "ref-term: cannot open %s\n", path );
    exit( 2 );
  }
  return read_all( f );
}

static std::vector<std::string> records( const std::string& in )
{
  std::vector<std::string> out;
  size_t p = 0;
  while ( p < in.size() ) {
    if ( in.size() - p < 4 ) {
      fprintf( stderr, "ref-term: truncated record header\n" );
      exit( 2 );
    }
    const unsigned char* h = (const unsigned char*)in.data() + p;
    size_t len = ( (size_t)h[0] << 24 ) | ( (size_t)h[1] << 16 ) | ( (size_t)h[2] << 8 ) | h[3];
    p += 4;
    if ( in.size() - p < len ) {
      fprintf( stderr, "ref-term: truncated record\n" );
      exit( 2 );
    }
    out.push_back( in.substr( p, len ) );
    p += len;
  }
  return out;
}

static std::string do_dump( int w, int h, const std::string& input )
{
  Emulator emu( w, h );
  Parser::UTF8Parser parser;
  std::string to_host, o;
  feed( parser, emu, input, to_host );
  dump( emu.get_fb(), to_host, o );
  return o;
}

/* steps: one emulator; records tagged 'B' (bytes to feed) or 'R' (u32 BE
   width, u32 BE height: a Parser::Resize action, as Complete::act(Action)). */
static std::string do_steps( int w, int h, const std::vector<std::string>& recs )
{
  Emulator emu( w, h );
  Parser::UTF8Parser parser;
  std::string to_host, o;
  for ( const std::string& r : recs ) {
    if ( r.size() >= 1 && r[0] == 'B' ) {
      feed( parser, emu, r.substr( 1 ), to_host );
    } else if ( r.size() == 9 && r[0] == 'R' ) {
      const unsigned char* p = (const unsigned char*)r.data() + 1;
      size_t nw = ( (size_t)p[0] << 24 ) | ( (size_t)p[1] << 16 ) | ( (size_t)p[2] << 8 ) | p[3];
      size_t nh = ( (size_t)p[4] << 24 ) | ( (size_t)p[5] << 16 ) | ( (size_t)p[6] << 8 ) | p[7];
      if ( nw < 1 || nh < 1 ) {
        fprintf( stderr, "ref-term: resize to zero\n" );
        exit( 2 );
      }
      Parser::Resize act( nw, nh );
      act.act_on_terminal( &emu );
      to_host += emu.read_octets_to_host();
    } else {
      fprintf( stderr, "ref-term: bad steps record\n" );
      exit( 2 );
    }
  }
  dump( emu.get_fb(), to_host, o );
  return o;
}

/* frame-resize: like do_frame, but the emulator is resized to w2 x h2 (a
   Parser::Resize action) between the old and the new bytes. */
static std::string do_frame_resize( int w,
                                    int h,
                                    size_t w2,
                                    size_t h2,
                                    const std::string& oldb,
                                    const std::string& newb,
                                    bool initialized )
{
  Emulator emu( w, h );
  Parser::UTF8Parser parser;
  std::string to_host;
  feed( parser, emu, oldb, to_host );
  Framebuffer last( emu.get_fb() );
  Parser::Resize act( w2, h2 );
  act.act_on_terminal( &emu );
  feed( parser, emu, newb, to_host );
  Display display( false );
  return display.new_frame( initialized, last, emu.get_fb() );
}

static std::string do_frame( int w, int h, const std::string& oldb, const std::string& newb, bool initialized )
{
  Emulator emu( w, h );
  Parser::UTF8Parser parser;
  std::string to_host;
  feed( parser, emu, oldb, to_host );
  Framebuffer last( emu.get_fb() );
  feed( parser, emu, newb, to_host );
  Display display( false );
  return display.new_frame( initialized, last, emu.get_fb() );
}

static void out( const std::string& s )
{
  fwrite( s.data(), 1, s.size(), stdout );
}

static std::string capstr( const char* s )
{
  return s ? hex( s ) : std::string( "none" );
}

/* caps: for each name on stdin, upstream's Display(true) decisions under
   TERM=<name> (setupterm and the TERM title whitelist), one line each. */
static void do_caps( void )
{
  std::string name;
  while ( std::getline( std::cin, name ) ) {
    setenv( "TERM", name.c_str(), 1 );
    std::string line = hex( name ) + " ";
    try {
      Display d( true );
      line += std::string( "ech " ) + ( d.has_ech ? "1" : "0" ) + " bce " + ( d.has_bce ? "1" : "0" ) + " title "
              + ( d.has_title ? "1" : "0" ) + " smcup " + capstr( d.smcup ) + " rmcup " + capstr( d.rmcup );
      /* Display(true) ran setupterm for this TERM; mosh-client -c reads: */
      line += " colors " + std::to_string( tigetnum( "colors" ) );
    } catch ( const std::exception& e ) {
      line += std::string( "error " ) + e.what();
    }
    out( line + "\n" );
  }
}

static void usage( void )
{
  fprintf( stderr,
           "usage: ref-term dump W H < input\n"
           "       ref-term caps < terminal type names\n"
           "       ref-term steps W H < tagged records\n"
           "       ref-term batch W H < records\n"
           "       ref-term frame W H OLDFILE NEWFILE [initialized]\n"
           "       ref-term frame-batch W H [initialized] < record pairs\n"
           "       ref-term frame-resize-batch W H [initialized] < (size, old, new) triples\n" );
  exit( 2 );
}

int main( int argc, char** argv )
{
  if ( NULL == setlocale( LC_ALL, "C.UTF-8" ) ) {
    fprintf( stderr, "ref-term: setlocale(C.UTF-8) failed\n" );
    return 1;
  }
  if ( argc == 2 && std::string( argv[1] ) == "caps" ) {
    do_caps();
    return 0;
  }
  if ( argc < 4 ) {
    usage();
  }
  std::string mode = argv[1];
  int w = atoi( argv[2] ), h = atoi( argv[3] );
  if ( w < 1 || h < 1 ) {
    usage();
  }

  if ( mode == "dump" && argc == 4 ) {
    out( do_dump( w, h, read_all( std::cin ) ) );
  } else if ( mode == "batch" && argc == 4 ) {
    for ( const std::string& r : records( read_all( std::cin ) ) ) {
      out( do_dump( w, h, r ) );
      out( "=====\n" );
    }
  } else if ( mode == "steps" && argc == 4 ) {
    out( do_steps( w, h, records( read_all( std::cin ) ) ) );
  } else if ( mode == "frame" && ( argc == 6 || argc == 7 ) ) {
    bool initialized = argc == 7 ? atoi( argv[6] ) != 0 : true;
    out( do_frame( w, h, read_file( argv[4] ), read_file( argv[5] ), initialized ) );
  } else if ( mode == "frame-resize-batch" && ( argc == 4 || argc == 5 ) ) {
    bool initialized = argc == 5 ? atoi( argv[4] ) != 0 : true;
    std::vector<std::string> recs = records( read_all( std::cin ) );
    if ( recs.size() % 3 ) {
      fprintf( stderr, "ref-term: frame-resize-batch needs records in threes\n" );
      return 2;
    }
    for ( size_t i = 0; i < recs.size(); i += 3 ) {
      const unsigned char* p = (const unsigned char*)recs[i].data();
      if ( recs[i].size() != 8 ) {
        fprintf( stderr, "ref-term: size record must be 8 bytes\n" );
        return 2;
      }
      size_t w2 = ( (size_t)p[0] << 24 ) | ( (size_t)p[1] << 16 ) | ( (size_t)p[2] << 8 ) | p[3];
      size_t h2 = ( (size_t)p[4] << 24 ) | ( (size_t)p[5] << 16 ) | ( (size_t)p[6] << 8 ) | p[7];
      if ( w2 < 1 || h2 < 1 ) {
        fprintf( stderr, "ref-term: resize to zero\n" );
        return 2;
      }
      std::string f = do_frame_resize( w, h, w2, h2, recs[i + 1], recs[i + 2], initialized );
      unsigned char hdr[4] = { (unsigned char)( f.size() >> 24 ),
                               (unsigned char)( f.size() >> 16 ),
                               (unsigned char)( f.size() >> 8 ),
                               (unsigned char)f.size() };
      fwrite( hdr, 1, 4, stdout );
      out( f );
    }
  } else if ( mode == "frame-batch" && ( argc == 4 || argc == 5 ) ) {
    bool initialized = argc == 5 ? atoi( argv[4] ) != 0 : true;
    std::vector<std::string> recs = records( read_all( std::cin ) );
    if ( recs.size() % 2 ) {
      fprintf( stderr, "ref-term: frame-batch needs an even number of records\n" );
      return 2;
    }
    for ( size_t i = 0; i < recs.size(); i += 2 ) {
      std::string f = do_frame( w, h, recs[i], recs[i + 1], initialized );
      unsigned char hdr[4] = { (unsigned char)( f.size() >> 24 ),
                               (unsigned char)( f.size() >> 16 ),
                               (unsigned char)( f.size() >> 8 ),
                               (unsigned char)f.size() };
      fwrite( hdr, 1, 4, stdout );
      out( f );
    }
  } else {
    usage();
  }
  return 0;
}
