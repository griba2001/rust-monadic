// mod mio

use std::io;
use std::io::prelude::*;


pub fn read_line() -> io::Result<String> {
   let stdin = io::stdin();
   let handle = &mut stdin.lock();
   let mut buf = String::new();
   let _bytes = handle.read_line( &mut buf)?;
   buf.pop(); // pop newline char
   Ok(buf)
   }
   
pub fn print_str( s: &str) -> io::Result<()> {
   let stdout = io::stdout();
   let handle = &mut stdout.lock();
   handle.write_all(s.as_bytes())?;
   Ok(())
}

pub fn print_string( s: &String) -> io::Result<()> {
   let stdout = io::stdout();
   let handle = &mut stdout.lock();
   handle.write_all(s.as_bytes())?;
   Ok(())
}

pub fn stdout_flush() -> io::Result<()> {
   let stdout = io::stdout();
   let handle = &mut stdout.lock();
   handle.flush()?;
   Ok(())
}

//-------------------------------------------------------------------

/*
use std::fs::{File, OpenOptions};
use std::path::Path;

pub fn with_file<A>( get_resource: fn(()) -> io::Result<File>, 
                  file_action: fn(&mut File) -> io::Result<A>) 
                  -> io::Result<A> {
   let mut handle = get_resource( ())? ;
   file_action( &mut handle)
}

pub fn open_rd( fname: &str) -> io::Result<File> {
   let path = Path::new( fname);
   File::open( path)
}

pub fn open_wr( fname: &str) -> io::Result<File> {
   let path = Path::new( fname);
   OpenOptions::new().write(true)
                     .truncate(true) // truncate if it exists 
                     .create(true)   // create if it does'nt exist
                     .open( path)
}
*/
