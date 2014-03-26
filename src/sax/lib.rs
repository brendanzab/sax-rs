// Copyright 2013 The SAX-RS Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[comment = "Wrapper for libxml2's SAX parser."];
#[crate_id = "github.com/bjz/sax-rs#sax:0.1"];

#[crate_type = "lib"];

#[feature(globs)];

extern crate sync;

use std::cast;
use std::comm::{Receiver, channel};
use std::libc::{c_char, c_int};
use std::str;
use std::io::{File, IoResult};
use std::fmt;

use error::ErrorData;

pub mod error;
pub mod ffi;
mod extfn;

/// Events to be sent by the parser.
#[deriving(Eq, Clone)]
pub enum ParseEvent {
    /// The document has begun to be processed.
    StartDocument,
    /// The document processing has finished.
    EndDocument,
    /// An opening tag has was parsed.
    StartElement(~str, Attributes),
    /// A closing tag was parsed.
    EndElement(~str),
    /// Some characters between tags have been recived.
    Characters(~str),
    /// A comment tag was parsed.
    Comment(~str),
    /// A `CDATA` block was parsed.
    CdataBlock(~str),
}

impl fmt::Show for ParseEvent {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartDocument => write!(fmt.buf, "START DOCUMENT"),
            EndDocument => write!(fmt.buf, "END DOCUMENT"),
            StartElement(ref name, ref atts) => write!(fmt.buf, "<{}{}>", *name, atts),
            EndElement(ref name) => write!(fmt.buf, "</{}>", *name),
            Characters(ref ch) => write!(fmt.buf, "{}", ch.clone()),
            Comment(ref value) => write!(fmt.buf, "<!--{}-->", *value),
            CdataBlock(ref value) => write!(fmt.buf, "<![CDATA[{}]]>", *value),
        }
    }
}

#[deriving(Eq, Clone)]
pub struct Attribute {
    name: ~str,
    value: ~str,
}

/// A list of attributes
#[deriving(Eq, Clone)]
pub struct Attributes(Vec<Attribute>);

impl Attributes {
    unsafe fn from_buf(atts: **ffi::xmlChar) -> Attributes {
        let mut ret = Vec::new();
        let mut ptr = atts as **c_char;
        while !ptr.is_null() && !(*ptr).is_null() {
            ret.push(
                Attribute {
                    name: str::raw::from_c_str(*ptr),
                    value: str::raw::from_c_str(*ptr.offset(1)),
                }
            );
            ptr = ptr.offset(2);
        }
        Attributes(ret)
    }

    pub fn find<'a>(&'a self, name: &str) -> Option<&'a str> {
        let Attributes(ref s) = *self;
        s.iter()
         .find(|att| name == att.name)
         .map(|att| att.value.as_slice())
    }

    pub fn get<'a>(&'a self, name: &str) -> &'a str {
        self.find(name).expect(format!("Could not find an attribute with the name \"{}\"", name))
    }

    pub fn find_clone(&self, name: &str) -> Option<~str> {
        self.find(name).map(|v| v.to_owned())
    }

    pub fn get_clone(&self, name: &str) -> ~str {
        self.get(name).to_owned()
    }
}

impl fmt::Show for Attributes {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let Attributes(ref s) = *self;
        for att in s.iter() {
            try!(write!(fmt.buf, " {}=\"{}\"", att.name, att.value));
        }
        Ok(())
    }
}

/// Either a parse event wrapped in `Ok` or some Error data wrapped in `Err`.
pub type ParseResult = Result<ParseEvent, ErrorData>;

/// Parses the entire XML string.
///
/// # Returns
///
/// A receiver that recieves parse results.
///
/// # Example
///
/// ~~~rust
/// let parser = parse_str("<yo>hullo!</yo>");
/// for result in parser.iter() {
///     match result {
///         Ok(StartDocument) => (),
///         Ok(EndDocument) => break,
///         Ok(event) => println!("{}", event.to_str()),
///         Err(err) => println!("{}", err.to_str()),
///     }
/// }
/// ~~~
#[inline(never)]
pub fn parse_str(src: &str) -> Receiver<ParseResult> {
    // ensure that the xml library is ready for use
    use sync::one::{Once, ONCE_INIT};
    static mut INIT: Once = ONCE_INIT;
    unsafe {
        INIT.doit(|| {
            ffi::xmlInitParser();
            std::rt::at_exit(proc() ffi::xmlCleanupParser());
        })
    }

    let len = src.len() as c_int;
    src.to_c_str().with_ref(|c_str| {
        let (sender, receiver) = channel();
        unsafe {
            ffi::xmlSAXUserParseMemory(&extfn::new_handler(),
                                       cast::transmute(&sender),
                                       c_str, len);
        }
        receiver
    })
}

pub fn parse_file(path: &Path) -> IoResult<Receiver<ParseResult> > {
  File::open(path)
    .and_then( |mut file| {
      file.read_to_str()
    })
    .and_then( |contents| {
      Ok(parse_str(contents))
    })
}

#[cfg(test)]
mod tests {
    use std::vec_ng::Vec;
    use super::*;

    fn get_mock_atts() -> Attributes {
        Attributes(Vec::from_slice([
            Attribute { name: ~"foo", value: ~"0" },
            Attribute { name: ~"bar", value: ~"1" },
            Attribute { name: ~"baz", value: ~"2" }
        ]))
    }

    #[test]
    fn test_att_get() {
        let atts = get_mock_atts();
        assert_eq!(atts.get("foo"), "0");
        assert_eq!(atts.get("bar"), "1");
        assert_eq!(atts.get("baz"), "2");
    }

    #[test]
    fn test_att_find() {
        let atts = get_mock_atts();
        assert_eq!(atts.find("foo"), Some("0"));
        assert_eq!(atts.find("bar"), Some("1"));
        assert_eq!(atts.find("baz"), Some("2"));
    }

    #[test]
    fn test() {
        let sax = parse_str(
            "<hello><this /><a foo=\"bar\">test</a></hello>"
        );
        loop {
            match sax.recv() {
                Ok(StartDocument) => (),
                Ok(EndDocument) => break,
                Ok(event) => println!("{}", event.to_str()),
                Err(err) => println!("{}", err.to_str()),
            }
        }
    }
}
