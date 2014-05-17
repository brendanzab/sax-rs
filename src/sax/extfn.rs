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

//! External callback definitions

use libc::{c_char, c_int, c_void};
use std::mem::transmute;
use std::comm::Sender;
use std::ptr::null;
use std::str::raw::{from_c_str, from_buf_len};

use super::*;
use super::error::ErrorData;

pub fn new_handler() -> ffi::xmlSAXHandler {
    ffi::xmlSAXHandler {
        internalSubset:         None,
        isStandalone:           None,
        hasInternalSubset:      None,
        hasExternalSubset:      None,
        resolveEntity:          None,
        getEntity:              None,
        entityDecl:             None,
        notationDecl:           None,
        attributeDecl:          None,
        elementDecl:            None,
        unparsedEntityDecl:     None,
        setDocumentLocator:     None,
        startDocument:          Some(start_document),
        endDocument:            Some(end_document),
        startElement:           Some(start_element),
        endElement:             Some(end_element),
        reference:              None,
        characters:             Some(characters),
        ignorableWhitespace:    None,               // use characters
        processingInstruction:  None,
        comment:                Some(comment),
        warning:                None,               // use serror
        error:                  None,               // use serror
        fatalError:             None,               // use serror
        getParameterEntity:     None,
        cdataBlock:             Some(cdata_block),
        externalSubset:         None,
        initialized:            ffi::XML_SAX2_MAGIC,
        _private:               null(),
        startElementNs:         None,
        endElementNs:           None,
        serror:                 Some(serror),
    }
}

unsafe fn sender_from_ptr(ctx: *c_void) -> &Sender<ParseResult> { transmute(ctx) }

extern "C" fn start_document(ctx: *c_void) {
    unsafe {
        sender_from_ptr(ctx).send(
            Ok(StartDocument)
        );
    }
}

extern "C" fn end_document(ctx: *c_void) {
    unsafe {
        sender_from_ptr(ctx).send(
            Ok(EndDocument)
        );
    }
}

extern "C" fn start_element(ctx: *c_void, name: *ffi::xmlChar, atts: **ffi::xmlChar) {
    unsafe {
        sender_from_ptr(ctx).send(
            Ok(StartElement(from_c_str(name as *c_char).to_strbuf(), Attributes::from_buf(atts)))
        );
    }
}

extern "C" fn end_element(ctx: *c_void, name: *ffi::xmlChar) {
    unsafe {
        sender_from_ptr(ctx).send(
            Ok(EndElement(from_c_str(name as *c_char).to_strbuf()))
        );
    }
}

extern "C" fn characters(ctx: *c_void, ch: *ffi::xmlChar, len: c_int) {
    unsafe {
        sender_from_ptr(ctx).send(
            Ok(Characters(from_buf_len(ch, len as uint).to_strbuf()))
        );
    }
}

extern "C" fn comment(ctx: *c_void, value: *ffi::xmlChar) {
    unsafe {
        sender_from_ptr(ctx).send(
            Ok(Comment(from_c_str(value as *c_char).to_strbuf()))
        );
    }
}

extern "C" fn cdata_block(ctx: *c_void, value: *ffi::xmlChar, len: c_int) {
    unsafe {
        sender_from_ptr(ctx).send(
            Ok(CdataBlock(from_buf_len(value, len as uint).to_strbuf()))
        );
    }
}

extern "C" fn serror(ctx: *c_void, error: *ffi::xmlError) {
    unsafe {
        ErrorData::from_ptr(error).map(|err| {
            sender_from_ptr(ctx).send(Err(err.clone()));
        });
    }
}
