use gio::glib::value::{FromValue, FromValueOptional, ToValueOptional};
use gio::glib::Error;
use gio::prelude::*;
use gio::subclass::prelude::OutputStreamImpl;
use gio::{glib, Cancellable, OutputStream};
use glib::subclass::prelude::*;
use glib::translate::*;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::io::Write;

mod imp {
    use super::*;
    use gio::glib::Error;
    use gio::subclass::prelude::OutputStreamImpl;
    use gio::IOErrorEnum::NotFound;
    use std::cell::RefCell;

    #[derive(Default)]
    pub struct ChunkedOutputStream {
        pub(super) inner: RefCell<Option<OutputStream>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ChunkedOutputStream {
        const NAME: &'static str = "ChunkedOutputStream";
        type Type = super::ChunkedOutputStream;
        type ParentType = OutputStream;
    }

    impl ObjectImpl for ChunkedOutputStream {}

    impl OutputStreamImpl for ChunkedOutputStream {
        fn write(
            &self,
            chunk: &[u8],
            _cancellable: Option<&Cancellable>,
        ) -> Result<usize, Error> {
            if let Some(ref mut stream) = *self.inner.borrow_mut() {
                // Write the chunk size in hex followed by CRLF
                let chunk_size = format!("{:X}\r\n", chunk.len());
                stream.write_all(chunk_size.as_bytes(), None::<&Cancellable>).unwrap();

                // Write the actual chunk data
                stream.write_all(chunk, None::<&Cancellable>).unwrap();

                // Write a CRLF after the chunk data
                stream.write_all(b"\r\n", None::<&Cancellable>).unwrap();

                Ok(chunk.len())
            } else {
                Err(Error::new(NotFound, "No output stream available"))
            }
        }

        fn close(&self, cancellable: Option<&Cancellable>) -> Result<(), Error> {
            if let Some(ref mut stream) = *self.inner.borrow_mut() {
                // Write the final chunk (0) to signal end of stream
                stream.write_all(b"0\r\n\r\n", None::<&Cancellable>).unwrap();

                Ok(())
            } else {
                Err(Error::new(NotFound, "No output stream available"))
            }
        }
    }
    unsafe impl Send for ChunkedOutputStream {}
}

glib::wrapper! {
        pub struct ChunkedOutputStream(ObjectSubclass<imp::ChunkedOutputStream>)
            @extends OutputStream;
}

impl ChunkedOutputStream {
    pub fn new(output_stream: OutputStream) -> ChunkedOutputStream {
        let stream = glib::Object::new::<ChunkedOutputStream>();

        *stream.imp().inner.borrow_mut() = Some(output_stream);
        stream
    }
}