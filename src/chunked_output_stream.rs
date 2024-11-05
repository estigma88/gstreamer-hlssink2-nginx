use std::borrow::Borrow;
use gio::prelude::*;
use gio::{glib, Cancellable, OutputStream};
use glib::subclass::prelude::*;
use glib::translate::*;
use std::io::{self, Write};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use gio::glib::object::ObjectRef;
use gio::glib::{Type, Value};
use gio::glib::value::{FromValue, FromValueOptional, ToValueOptional};
use gio::subclass::prelude::OutputStreamImpl;
use crate::chunked_output_stream::imp::ChunkedOutputStream;

mod imp {
    use std::cell::RefCell;
    use std::io;
    use gio::{glib, Cancellable, OutputStream};
    use gio::prelude::OutputStreamExtManual;
    use gio::subclass::prelude::{ObjectImpl, ObjectSubclass, OutputStreamImpl};

    pub struct ChunkedOutputStream {
        pub(crate) inner: RefCell<gio::OutputStream>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ChunkedOutputStream {
        const NAME: &'static str = "ChunkedOutputStream";
        type Type = super::ChunkedOutputStream;
        type ParentType = OutputStream;
        type Interfaces = ();
    }

    impl ObjectImpl for ChunkedOutputStream {}

    impl OutputStreamImpl for ChunkedOutputStream {
        fn write(&self, buffer: &[u8], _cancellable: Option<&gio::Cancellable>) -> Result<usize, glib::Error> {
            // Implement the chunked writing
            match self.write_chunk(buffer) {
                Ok(len) => Ok(len),
                Err(err) => Err(glib::Error::new(gio::IOErrorEnum::Failed, &err.to_string())),
            }
        }

        fn close(&self, _cancellable: Option<&gio::Cancellable>) -> Result<(), glib::Error> {
            // Write the final chunk to indicate the end of the transfer
            match self.write_final_chunk() {
                Ok(()) => Ok(()),
                Err(err) => Err(glib::Error::new(gio::IOErrorEnum::Failed, &err.to_string())),
            }
        }
    }
}

glib::wrapper! {
    pub struct ChunkedOutputStream(ObjectSubclass<imp.ChunkedOutputStream>) @extends crate::OutputStream;
}


impl ChunkedOutputStream {
    pub fn new(inner: gio::OutputStream) -> Self {
        Self {
            inner: RefCell::new(inner),
        }
    }

    fn write_chunk(&self, chunk: &[u8]) -> Result<usize, io::Error> {
        let mut inner = self.inner.borrow_mut();

        // Write the chunk size in hex followed by CRLF
        let chunk_size = format!("{:X}\r\n", chunk.len());
        inner.write_all(chunk_size.as_bytes(), None::<&Cancellable>).unwrap();

        // Write the actual chunk data
        inner.write_all(chunk, None::<&Cancellable>).unwrap();

        // Write a CRLF after the chunk data
        inner.write_all(b"\r\n", None::<&Cancellable>).unwrap();

        Ok(chunk.len())
    }

    fn write_final_chunk(&self) -> Result<(), io::Error> {
        let mut inner = self.inner.borrow_mut();

        // Write the final chunk (0) to signal end of stream
        inner.write_all(b"0\r\n\r\n", None::<&Cancellable>).unwrap();
        Ok(())
    }
}