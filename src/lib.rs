//! To any `std::io::Read` implementor, add also `Iterator<Item=u8>` implementation.
//!
//! # Installation
//!
//! In `Cargo.toml` of your project add:
//!
//! ```toml
//! [dependencies]
//! read_iter = "1.0"
//! ```
//!
//! # Examples
//!
//! ```no_run
//! use std::fs::File;
//! use read_iter::ReadIter;
//!
//! let file = File::open("/tmp/test.txt").unwrap();
//! // "file" implements std::io::Read
//! let mut it = ReadIter::new(file);
//! // now "it" also implements std::io::Read
//! // and "&mut it" implements Iterator<Item=u8>
//! // also "it" has internal buffer, and implements std::io::BufRead
//! for byte in &mut it
//! {	// ...
//! }
//! // in case of i/o error, the iteration ends, and take_last_error() will return Err
//! it.take_last_error().unwrap();
//! ```

use std::{io, cmp};

const BUFFER_SIZE: usize = 4*1024;

/// Object that wraps `std::io::Read`, and also implements `std::io::Read`.
/// It also implements `std::io::BufRead` and `Iterator<Item=u8>`.
pub struct ReadIter<T> where T: io::Read
{	reader: T,
	err: Option<io::Error>,
	buffer: [u8; BUFFER_SIZE],
	len: usize,
	i: usize,
}

impl<T> ReadIter<T> where T: io::Read
{	pub fn new(reader: T) -> Self
	{	Self
		{	reader,
			err: None,
			buffer: [0; BUFFER_SIZE],
			len: 0,
			i: 0
		}
	}

	/// Iteration can end in 2 cases:
	///
	/// - end of stream reached
	/// - i/o error occured
	///
	/// If there was error, this function returns &Some(err).
	pub fn last_error(&self) -> &Option<io::Error>
	{	&self.err
	}

	/// Iteration can end in 2 cases:
	///
	/// - end of stream reached
	/// - i/o error occured
	///
	/// If there was error, this function returns Err(err), and clears the error state.
	pub fn take_last_error(&mut self) -> Result<(), io::Error>
	{	match self.err.take()
		{	Some(err) => Err(err),
			None => Ok(())
		}
	}
}

impl<T> Iterator for &mut ReadIter<T> where T: io::Read
{	type Item = u8;

	fn next(&mut self) -> Option<Self::Item>
	{	if self.i < self.len
		{	let i = self.i;
			self.i += 1;
			Some(self.buffer[i])
		}
		else
		{	match self.reader.read(&mut self.buffer)
			{	Err(err) =>
				{	self.err = Some(err);
					None
				}
				Ok(0) =>
				{	None
				}
				Ok(n) =>
				{	self.len = n;
					self.i = 1;
					Some(self.buffer[0])
				}
			}
		}
	}
}

impl<T> io::Read for ReadIter<T> where T: io::Read
{	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>
	{	if self.i < self.len
		{	let n = cmp::min(buf.len(), self.len-self.i);
			buf[.. n].copy_from_slice(&self.buffer[self.i .. self.i+n]);
			self.i += n;
			Ok(n)
		}
		else
		{	self.reader.read(buf)
		}
	}
}

impl<T> io::BufRead for ReadIter<T> where T: io::Read
{	fn fill_buf(&mut self) -> Result<&[u8], io::Error>
	{	if self.i >= self.len
		{	match self.reader.read(&mut self.buffer)
			{	Err(err) =>
				{	return Err(err);
				}
				Ok(n) =>
				{	self.len = n;
					self.i = 0;
				}
			}
		}
		Ok(&self.buffer[self.i .. self.i+self.len])
	}

	fn consume(&mut self, amt: usize)
	{	self.i += amt;
	}
}

#[cfg(test)]
mod tests
{	use super::*;

	#[test]
	fn test_1()
	{	let reader = r#"Hello"#.as_bytes();
		let mut it = &mut ReadIter::new(reader);
		assert_eq!(it.next(), Some(b'H'));
		assert_eq!(it.next(), Some(b'e'));
		assert_eq!(it.next(), Some(b'l'));
		assert_eq!(it.next(), Some(b'l'));
		assert_eq!(it.next(), Some(b'o'));
		assert_eq!(it.next(), None);
		it.take_last_error().unwrap();
	}

	#[test]
	fn test_2()
	{	use std::io::Read;
		let reader = r#"Hello"#.as_bytes();
		let it = &mut ReadIter::new(reader);
		let mut it2 = it.bytes().map(|b| b.unwrap()); // back to std::io::Read
		assert_eq!(it2.next(), Some(b'H'));
		assert_eq!(it2.next(), Some(b'e'));
		assert_eq!(it2.next(), Some(b'l'));
		assert_eq!(it2.next(), Some(b'l'));
		assert_eq!(it2.next(), Some(b'o'));
		assert_eq!(it2.next(), None);
		it.take_last_error().unwrap();
	}
}
