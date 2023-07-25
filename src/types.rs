/*!
Machinery for filtering by entry type.
*/
use std::fs::FileType;
#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;
#[cfg(windows)]
use std::os::windows::fs::FileTypeExt;
#[cfg(wasi)]
use std::os::wasi::fs::FileTypeExt;

use clap::Parser;

/// Type of directory entry.
#[derive(Clone, Copy, Debug, ValueEnum)]
#[non_exhaustive]
pub enum EType {
    /// regular file
    File,
    /// directory
    Dir,
    /// symbolic link
    Link,
    /// FIFO (i.e., a pipe)
    #[cfg(unix)]
    Fifo,
    /// a socket
    #[cfg(any(unix, wasi))]
    Socket,
    /// block device
    #[cfg(any(unix, wasi))]
    Block,
    /// character device
    #[cfg(any(unix, wasi))]
    Char,
}

pub(crate) trait HasEType {
    fn is(&self, entry_type: &EType) -> bool;

    fn is_one<I, A>(&self, collection: I) -> bool
    where
        A: AsRef<EType>,
        I: IntoIterator<Item = A>
    {
        for t in collection.into_iter() {
            if self.is(t.as_ref()) { return true; }
        }

        false
    }
}

impl HasEType for FileType {
    fn is(&self, t: &EType) -> bool {
        use EType::*;
        match t.clone() {
            File => self.is_file(),
            Dir => self.is_dir(),
            Link => self.is_symlink(),
            #[cfg(unix)]
            Fifo => self.is_fifo(),
            #[cfg(unix)]
            Socket => self.is_socket(),
            #[cfg(wasi)]
            Socket => self.is_socket_dgram() || self.is_socket_stream(),
            #[cfg(any(unix, wasi))]
            Block => self.is_block_device(),
            #[cfg(any(unix, wasi))]
            Char => self.is_char_device(),
        }
    }
}