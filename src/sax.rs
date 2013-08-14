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

pub mod error;
pub mod ffi;
mod extfn;

#[deriving(Eq, Clone)]
pub enum ParseMsg {
    StartDocument,
    EndDocument,
    StartElement(~str, Attributes),
    EndElement(~str),
    Characters(~str),
    IgnorableWhitespace(~str),
    Comment(~str),
    CdataBlock(~str),
}

impl ToStr for ParseMsg {
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

pub type ParseResult = Result<ParseMsg, error::ErrorData>;

pub struct Parser {
    priv port: Port<ParseResult>,
}

impl GenericPort<ParseResult> for Parser {
    /// Recives a new parse result. Fails if the parser has finished. Failure
    /// can be avoided by finishing after recieving the `Ok(EndDocument)` result.
    pub fn recv(&self) -> ParseResult {
        self.port.try_recv().expect(
            "Could not get a new parse result, the parser has already finished!"
        )
    }

    pub fn try_recv(&self) -> Option<ParseResult> {
        self.port.try_recv()
    }
}

pub fn parse(src: &str) -> Parser {
    let (port, chan) = stream();
    unsafe {
        do src.to_c_str().with_ref |c_str| {
            ffi::xmlSAXUserParseMemory(&extfn::new_handler(),
                                       cast::transmute(&chan),
                                       c_str,
                                       src.len() as c_int);
            ffi::xmlCleanupParser();
        }
    }
    Parser { port: port }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_XML: &'static str = "<hello><this /><a foo=\"bar\">test</a></hello>";

    #[test]
    fn test() {
        let sax = parse(TEST_XML);
        loop {
            let msg = sax.recv().unwrap();
            println(msg.to_str());
            if msg == EndDocument { break }
        }
    }
}
