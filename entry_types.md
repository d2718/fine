The file type is one of the following characters:

`-` regular file

`b` block special file

`c` character special file

`C` high performance ("contiguous data") file

`d` directory

`D` door (Solaris 2.5 and up)

`l` symbolic link

`M` off-line ("migrated") file (Cray DMF)

`n` network special file (HP-UX)

`p` FIFO (named pipe)

`P` port (Solaris 10 and up)

`s` socket

`?` some other file type

```rust
std::fs::Filetype
    fn is_dir(&self) -> bool;
    fn is_file(&self) -> bool;
    fn is_symlink(&self) -> bool;
```

```rust
std::os::unix::fs::FileTypeExt
    fn is_block_device(&self) -> bool;
    fn is_char_device(&self) -> bool;
    fn is_fifo(&self) -> bool;
    fn is_socket(&self) -> bool;
```

```rust
std::os::windows::fs::FileTypeExt
    fn is_symlink_dir(&self) -> bool;
    fn is_symlink_file(&self) -> bool;
```

```rust
std::os::wasi::fs::FileTypeExt
    fn is_block_device(&self) -> bool;
    fn is_char_device(&self) -> bool;
    fn is_socket_dgram(&self) -> bool;
    fn is_socket_stream(&self) -> bool;
```