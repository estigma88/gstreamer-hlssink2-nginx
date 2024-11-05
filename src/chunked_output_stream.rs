use gio::glib::object::ObjectRef;
use gio::glib::value::{FromValue, FromValueOptional, ToValueOptional};
use gio::glib::{Error, Type, Value};
use gio::prelude::*;
use gio::subclass::prelude::OutputStreamImpl;
use gio::{glib, Cancellable, OutputStream};
use glib::subclass::prelude::*;
use glib::translate::*;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::io::{self, Write};
use std::sync::Mutex;

mod imp {
    use super::*;
    use gio::glib::Error;
    use gio::subclass::prelude::OutputStreamImpl;

    #[derive(Default)]
    pub struct ChunkedOutputStream {
        pub inner: Mutex<Option<OutputStream>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ChunkedOutputStream {
        const NAME: &'static str = "SimpleOutputStream";
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
            let inner = self.inner.lock().unwrap().take().unwrap();

            // Write the chunk size in hex followed by CRLF
            let chunk_size = format!("{:X}\r\n", chunk.len());
            inner.write_all(chunk_size.as_bytes(), None::<&Cancellable>).unwrap();

            // Write the actual chunk data
            inner.write_all(chunk, None::<&Cancellable>).unwrap();

            // Write a CRLF after the chunk data
            inner.write_all(b"\r\n", None::<&Cancellable>).unwrap();

            Ok(chunk.len())
        }

        fn close(&self, cancellable: Option<&Cancellable>) -> Result<(), Error> {
            let inner = self.inner.lock().unwrap().take().unwrap();

            // Write the final chunk (0) to signal end of stream
            inner.write_all(b"0\r\n\r\n", None::<&Cancellable>).unwrap();
            Ok(())
        }
    }


    impl ChunkedOutputStream {
        pub fn new(inner: OutputStream) -> Self {
            Self {
                inner: Mutex::new(Some(inner)),
            }
        }
    }

    unsafe impl Send for ChunkedOutputStream {}
}

glib::wrapper! {
        pub struct ChunkedOutputStream(ObjectSubclass<imp::ChunkedOutputStream>)
            @extends OutputStream;
}
