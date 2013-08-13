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

use std::cast::transmute;
use std::comm::Chan;
use std::libc::{c_char, c_int, c_void};
use std::ptr::null;
use std::str::raw::{from_c_str, from_buf_len};

use super::*;
use super::error::ErrorData;

pub fn new_sax_handler() -> ffi::xmlSAXHandler {
    ffi::xmlSAXHandler {
        resolveEntity:          null(),
        internalSubset:         null(),
        externalSubset:         null(),
        getEntity:              null(),
        entityDecl:             null(),
        getParameterEntity:     null(),
        notationDecl:           null(),
        attributeDecl:          null(),
        elementDecl:            null(),
        unparsedEntityDecl:     null(),
        setDocumentLocator:     null(),
        startDocument:          start_document,
        endDocument:            end_document,
        startElement:           start_element,
        endElement:             end_element,
        reference:              null(),
        characters:             null(),
        ignorableWhitespace:    ignorable_whitespace,
        processingInstruction:  null(),
        comment:                comment,
        cdataBlock:             cdata_block,
        warning:                null(),         // use serror
        error:                  null(),         // use serror
        fatalError:             null(),         // use serror
        isStandalone:           null(),
        hasInternalSubset:      null(),
        hasExternalSubset:      null(),
        initialized:            ffi::XML_SAX2_MAGIC,
        _private:               null(),
        startElementNs:         null(),
        endElementNs:           null(),
        serror:                 null(),
    }
}

unsafe fn chan_from_ptr(ctx: *c_void) -> &Chan<ParseResult> { transmute(ctx) }

extern "C" fn start_document(ctx: *c_void) {
    unsafe { chan_from_ptr(ctx).send(Ok(StartDocument)); }
}

extern "C" fn end_document(ctx: *c_void) {
    unsafe { chan_from_ptr(ctx).send(Ok(EndDocument)); }
}

extern "C" fn start_element(ctx: *c_void, name: *ffi::xmlChar, atts: **ffi::xmlChar) {
    unsafe {
        chan_from_ptr(ctx).send(
            Ok(StartElement(from_c_str(name as *c_char), Attributes::from_buf(atts)))
        );
    }
}

extern "C" fn end_element(ctx: *c_void, name: *ffi::xmlChar) {
    unsafe {
        chan_from_ptr(ctx).send(
            Ok(EndElement(from_c_str(name as *c_char)))
        );
    }
}

extern "C" fn characters(ctx: *c_void, ch: *ffi::xmlChar, len: c_int) {
    unsafe {
        chan_from_ptr(ctx).send(
            Ok(Characters(from_buf_len(ch, len as uint)))
        );
    }
}

extern "C" fn ignorable_whitespace(ctx: *c_void, ch: *ffi::xmlChar, len: c_int) {
    unsafe {
        chan_from_ptr(ctx).send(
            Ok(IgnorableWhitespace(from_buf_len(ch, len as uint)))
        );
    }
}

extern "C" fn comment(ctx: *c_void, value: *ffi::xmlChar) {
    unsafe {
        chan_from_ptr(ctx).send(
            Ok(Comment(from_c_str(value as *c_char)))
        );
    }
}

extern "C" fn cdata_block(ctx: *c_void, value: *ffi::xmlChar, len: c_int) {
    unsafe {
        chan_from_ptr(ctx).send(
            Ok(CdataBlock(from_buf_len(value, len as uint)))
        );
    }
}

extern "C" fn serror(ctx: *c_void, error: *ffi::xmlError) {
    unsafe {
        do ErrorData::from_ptr(error).map |err| {
            chan_from_ptr(ctx).send(Err(err.clone()));
        };
    }
}
