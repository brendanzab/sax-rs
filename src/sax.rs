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

#[link(name = "sax",
       vers = "0.1",
       author = "Brendan Zabarauskas",
       url = "https://github.com/bjz/sax-rs")];

#[comment = "Wrapper for libxml2's SAX parser."];
#[crate_type = "lib"];

extern mod extra;

use std::cast;
use std::comm::{Port, stream};
use std::hashmap::HashMap;
use std::libc::{c_char, c_int};
use std::str;
use std::task;

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
    /// Ignorable whitespace.
    IgnorableWhitespace(~str),
    /// A comment tag was parsed.
    Comment(~str),
    /// A `CDATA` block was parsed.
    CdataBlock(~str),
}

impl ToStr for ParseEvent {
    fn to_str(&self) -> ~str {
        match *self {
            StartDocument => ~"START DOCUMENT",
            EndDocument => ~"END DOCUMENT",
            StartElement(ref name, ref atts) => fmt!("<%s%s>", *name, atts.to_str()),
            EndElement(ref name) => fmt!("</%s>", *name),
            Characters(ref ch) => ch.clone(),
            IgnorableWhitespace(_) => ~"",
            Comment(ref value) => fmt!("<!--%s-->", *value),
            CdataBlock(ref value) => fmt!("<![CDATA[%s]]>", *value),
        }
    }
}

/// A list of attributes stored in a hashmap.
#[deriving(Eq, Clone)]
pub struct Attributes(HashMap<~str, ~str>);

impl Attributes {
    unsafe fn from_buf(atts: **ffi::xmlChar) -> Attributes {
        let mut map = Attributes(HashMap::new());
        let mut ptr = atts as **c_char;
        while !ptr.is_null() && !(*ptr).is_null() {
            map.insert(str::raw::from_c_str(*ptr),
                       str::raw::from_c_str(*(ptr + 1)));
            ptr = ptr + 2;
        }
        map
    }
}

impl ToStr for Attributes {
    fn to_str(&self) -> ~str {
        do self.iter().map |(k, v)| {
            fmt!(" %s=\"%s\"", *k, *v)
        }.to_owned_vec().concat()
    }
}

/// Either a parse event wrapped in `Ok` or some Error data wrapped in `Err`.
pub type ParseResult = Result<ParseEvent, ErrorData>;

/// A port to recieve `ParseResult`s from the parser.
pub struct SaxPort {
    priv port: Port<ParseResult>,
}

impl GenericPort<ParseResult> for SaxPort {
    /// Recives a new parse message.
    ///
    /// # Failure
    ///
    /// Fails if the method is called again after the final `Ok(EndDocument)`
    /// parse result has been recived.
    pub fn recv(&self) -> ParseResult {
        self.port.try_recv().expect(
            "Could not get a new parse result, the parser has already finished!"
        )
    }

    /// Receives a parse result wrapped in `Some`, or `None` if the parser has
    /// finished.
    pub fn try_recv(&self) -> Option<ParseResult> {
        self.port.try_recv()
    }
}

/// Parses the entire XML string.
///
/// # Returns
///
/// A port that recieves parse results.
///
/// # Example
///
/// ~~~rust
/// let port = parse_xml("<hullo><world /></hullo>");
/// loop {
///     if port.recv() == Ok(EndDocument) { break }
/// }
/// ~~~
pub fn parse_xml(src: &str) -> SaxPort {
    let len = src.len() as c_int;
    do src.to_c_str().with_ref |c_str| {
        let (port, chan) = stream();
        do task::spawn {
            unsafe {
                ffi::xmlSAXUserParseMemory(&extfn::new_handler(),
                                           cast::transmute(&chan),
                                           c_str, len);
                ffi::xmlCleanupParser();
            }
        }
        SaxPort { port: port }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let sax = parse_xml(
            "<hello><this /><a foo=\"bar\">test</a></hello>"
        );
        loop {
            match sax.recv() {
                Ok(StartDocument) => (),
                Ok(EndDocument) => break,
                Ok(event) => println(event.to_str()),
                Err(err) => println(err.to_str()),
            }
        }
    }
}
