use std::cell::Cell;
use std::marker::Copy;
use std::thread_local;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum Error {
    Success,
    FileNotFound,
    IsDirectory,
    WriteFail,
    ReadFail,
}

impl Error {
    thread_local! {
        static LAST_ERROR: Cell<Error> = const { Cell::new(Error::Success) };
    }

    fn last() -> Self {
        Self::LAST_ERROR.with(|cell| cell.get())
    }

    fn make_last(self) {
        Self::LAST_ERROR.with(|cell| cell.set(self));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Error::last(), Error::Success); // Initially Success

        Error::FileNotFound.make_last(); // Set the last error
        assert_eq!(Error::last(), Error::FileNotFound); // Check the stored error

        Error::Success.make_last(); // Reset to Success
        assert_eq!(Error::last(), Error::Success); // Check the reset
    }
}
