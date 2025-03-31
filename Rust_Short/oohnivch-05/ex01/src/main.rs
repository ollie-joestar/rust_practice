use std::sync::{Arc, Mutex};
use std::{io, thread};

struct Logger<W> {
    buffer: Box<[u8]>,
    writer: W,
    pos: usize,
}

impl<W> Logger<W> {
    pub fn new(threshold: usize, writer: W) -> Self {
        Logger {
            buffer: vec![0; threshold].into_boxed_slice(),
            writer,
            pos: 0,
        }
    }
}

impl<W: io::Write> Logger<W> {
    pub fn log(&mut self, message: &str) -> io::Result<()> {
        let message_nl = format!("{}\n", message);
        let bytes = message_nl.as_bytes();

        if self.pos + bytes.len() > self.buffer.len() {
            self.flush()?;
            self.writer.write_all(bytes)?;
        } else {
            self.buffer[self.pos..self.pos + bytes.len()].copy_from_slice(bytes);
            self.pos += bytes.len();
        };
        if self.pos == self.buffer.len() {
            self.flush()?;
        }
        Ok(())
    }
    pub fn flush(&mut self) -> io::Result<()> {
        if self.pos > 0 {
            let _ = self.writer.write_all(&self.buffer[..self.pos]);
            self.buffer = vec![0; self.buffer.len()].into_boxed_slice();
            self.pos = 0;
        }
        Ok(())
    }
}

fn main() {
    let logger = Arc::new(Mutex::new(Logger::new(0, std::io::stdout())));
    let mut handles = vec![];

    for t in 0..10 {
        let logger = Arc::clone(&logger);
        let handle = thread::spawn(move || {
            for i in 0..10 {
                {
                    let mut t_log = logger.lock().unwrap();
                    let message: String = format!("hello {} from thread {}!", 4 + i % 2, t);
                    let _ = t_log.log(&message);
                }
                for _ in 0..10000 {}
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let _ = logger.lock().unwrap().flush();
}

#[cfg(test)]
mod tests {
    use super::*;
    //use std::io::Write;

    #[test]
    fn test_log_exact_buffer_size() {
        // Arrange: Create a buffer size of 5 (including the newline).
        let mut writer = Vec::new();
        let mut logger = Logger::new(5, &mut writer);

        // Act: Log a message with length exactly matching the buffer size.
        logger.log("1234").unwrap(); // "1234\n" is 5 bytes.
                                     //logger.flush().unwrap();
                                     //// Ensure the buffer is flushed.

        // Assert: Verify that the content matches what we expect.
        assert_eq!(writer, b"1234\n");
    }
    #[test]
    fn test_empty_unlogged_message() {
        // Arrange: Create a buffer size of 5 (including the newline).
        let mut writer = Vec::new();
        let mut logger = Logger::new(1000, &mut writer);

        // Act: Log a message with length exactly matching the buffer size.
        logger.log("1234").unwrap(); // "1234\n" is 5 bytes.
        assert_eq!(logger.writer, b"");
        //// Ensure the buffer is flushed.
        logger.flush().unwrap();

        // Assert: Verify that the content matches what we expect.
        assert_eq!(writer, b"1234\n");
    }
}
