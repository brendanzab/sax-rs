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

//! Foreign Function bindings for `libxml2`. These definitions can be found in
//! the `libxml/parser.h` header file.

use std::libc::*;

pub type xmlChar = c_uchar;

/// Special constant found in SAX2 blocks initialized fields
pub static XML_SAX2_MAGIC: c_uint = 0xDEEDBEAF;

pub type resolveEntitySAXFunc           = extern "C" fn(ctx: *c_void, publicId: *xmlChar, systemId: *xmlChar) -> *c_void /*xmlParserInputPtr*/;
pub type internalSubsetSAXFunc          = extern "C" fn(ctx: *c_void, name: *xmlChar, ExternalID: *xmlChar, SystemID: *xmlChar);
pub type externalSubsetSAXFunc          = extern "C" fn(ctx: *c_void, name: *xmlChar, ExternalID: *xmlChar, SystemID: *xmlChar);
pub type getEntitySAXFunc               = extern "C" fn(ctx: *c_void, name: *xmlChar) -> *c_void /*xmlEntityPtr*/;
pub type getParameterEntitySAXFunc      = extern "C" fn(ctx: *c_void, name: *xmlChar) -> *c_void /*xmlEntityPtr*/;
pub type entityDeclSAXFunc              = extern "C" fn(ctx: *c_void, name: *xmlChar, ty: c_int, publicId: *xmlChar, systemId: *xmlChar, content: *xmlChar);
pub type notationDeclSAXFunc            = extern "C" fn(ctx: *c_void, name: *xmlChar, publicId: *xmlChar, systemId: *xmlChar);
pub type attributeDeclSAXFunc           = extern "C" fn(ctx: *c_void, elem: *xmlChar, fullname: *xmlChar, ty: c_int, def: c_int, defaultValue: *xmlChar, tree: *c_void /*xmlEnumerationPtr*/);
pub type elementDeclSAXFunc             = extern "C" fn(ctx: *c_void, name: *xmlChar, ty: c_int, content: *c_void /*xmlElementContentPtr*/);
pub type unparsedEntityDeclSAXFunc      = extern "C" fn(ctx: *c_void, name: *xmlChar, publicId: *xmlChar, systemId: *xmlChar, notationName: *xmlChar);
pub type setDocumentLocatorSAXFunc      = extern "C" fn(ctx: *c_void, loc: *c_void /*xmlSAXLocatorPtr*/);
pub type startDocumentSAXFunc           = extern "C" fn(ctx: *c_void);
pub type endDocumentSAXFunc             = extern "C" fn(ctx: *c_void);
pub type startElementSAXFunc            = extern "C" fn(ctx: *c_void, name: *xmlChar, atts: **xmlChar);
pub type endElementSAXFunc              = extern "C" fn(ctx: *c_void, name: *xmlChar);
pub type referenceSAXFunc               = extern "C" fn(ctx: *c_void, name: *xmlChar);
pub type charactersSAXFunc              = extern "C" fn(ctx: *c_void, ch: *xmlChar, len: c_int);
pub type ignorableWhitespaceSAXFunc     = extern "C" fn(ctx: *c_void, ch: *xmlChar, len: c_int);
pub type processingInstructionSAXFunc   = extern "C" fn(ctx: *c_void, target: *xmlChar, data: *xmlChar);
pub type commentSAXFunc                 = extern "C" fn(ctx: *c_void, value: *xmlChar);
pub type cdataBlockSAXFunc              = extern "C" fn(ctx: *c_void, value: *xmlChar, len: c_int);
pub type warningSAXFunc                 = extern "C" fn(ctx: *c_void /*const char *msg, ...*/);
pub type errorSAXFunc                   = extern "C" fn(ctx: *c_void /*const char *msg, ...*/);
pub type fatalErrorSAXFunc              = extern "C" fn(ctx: *c_void /*const char *msg, ...*/);
pub type isStandaloneSAXFunc            = extern "C" fn(ctx: *c_void) -> int;
pub type hasInternalSubsetSAXFunc       = extern "C" fn(ctx: *c_void) -> int;
pub type hasExternalSubsetSAXFunc       = extern "C" fn(ctx: *c_void) -> int;
pub type startElementNsSAX2Func         = extern "C" fn(ctx: *c_void, localname: *xmlChar, prefix: *xmlChar, URI: *xmlChar, nb_namespaces: c_int, namespaces: **xmlChar, nb_attributes: c_int, nb_defaulted: c_int, attributes: **xmlChar);
pub type endElementNsSAX2Func           = extern "C" fn(ctx: *c_void, localname: *xmlChar, prefix: *xmlChar, URI: *xmlChar);
pub type xmlStructuredErrorFunc         = extern "C" fn(userData: *c_void, error: xmlErrorPtr);

pub struct xmlSAXHandler {
    internalSubset:         internalSubsetSAXFunc,
    isStandalone:           isStandaloneSAXFunc,
    hasInternalSubset:      hasInternalSubsetSAXFunc,
    hasExternalSubset:      hasExternalSubsetSAXFunc,
    resolveEntity:          resolveEntitySAXFunc,
    getEntity:              getEntitySAXFunc,
    entityDecl:             entityDeclSAXFunc,
    notationDecl:           notationDeclSAXFunc,
    attributeDecl:          attributeDeclSAXFunc,
    elementDecl:            elementDeclSAXFunc,
    unparsedEntityDecl:     unparsedEntityDeclSAXFunc,
    setDocumentLocator:     setDocumentLocatorSAXFunc,
    startDocument:          startDocumentSAXFunc,
    endDocument:            endDocumentSAXFunc,
    startElement:           startElementSAXFunc,
    endElement:             endElementSAXFunc,
    reference:              referenceSAXFunc,
    characters:             charactersSAXFunc,
    ignorableWhitespace:    ignorableWhitespaceSAXFunc,
    processingInstruction:  processingInstructionSAXFunc,
    comment:                commentSAXFunc,
    warning:                warningSAXFunc,
    error:                  errorSAXFunc,
    fatalError:             fatalErrorSAXFunc,
    getParameterEntity:     getParameterEntitySAXFunc,
    cdataBlock:             cdataBlockSAXFunc,
    externalSubset:         externalSubsetSAXFunc,
    initialized:            c_uint,
    _private:               *c_void,
    startElementNs:         startElementNsSAX2Func,
    endElementNs:           endElementNsSAX2Func,
    serror:                 xmlStructuredErrorFunc,
}

/// Error level type alias
pub type xmlErrorLevel = c_int;

/// No error
pub static XML_ERR_NONE:    xmlErrorLevel = 0;
/// A simple warning
pub static XML_ERR_WARNING: xmlErrorLevel = 1;
/// A recoverable error
pub static XML_ERR_ERROR:   xmlErrorLevel = 2;
/// A fatal error
pub static XML_ERR_FATAL:   xmlErrorLevel = 3;

pub struct xmlError {
    /// What part of the library raised this error
    domain: c_int,
    /// The error code, e.g. an xmlParserError
    code: c_int,
    /// human-readable informative error messag
    message: *c_char,
    /// how consequent is the error
    level: xmlErrorLevel,
    /// the filename
    file: *c_char,
    /// the line number if available
    line: c_int,
    /// extra string information
    str1: *c_char,
    /// extra string information
    str2: *c_char,
    /// extra string information
    str3: *c_char,
    /// extra number information
    int1: c_int,
    /// column number of the error or 0 if N/A
    int2: c_int,
    /// the parser context if available
    ctxt: *c_void,
    /// the node in the tree
    node: *c_void,
}

pub type xmlErrorPtr = *xmlError;

/// libxml2 function bindings
#[link_args="-lxml2"]
extern "C" {
    pub fn xmlCleanupParser();
    pub fn xmlSAXUserParseMemory(sax: *xmlSAXHandler,
                                 user_data: *c_void,
                                 buffer: *c_char,
                                 size: c_int) -> c_int;
}
