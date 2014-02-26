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

pub type XmlChar = c_uchar;

/// Special constant found in SAX2 blocks initialized fields
pub static XML_SAX2_MAGIC: c_uint = 0xDEEDBEAF;

pub type ResolveEntitySAXFunc           = extern "C" fn(ctx: *c_void, publicId: *XmlChar, systemId: *XmlChar) -> *c_void /*xmlParserInputPtr*/;
pub type InternalSubsetSAXFunc          = extern "C" fn(ctx: *c_void, name: *XmlChar, ExternalID: *XmlChar, SystemID: *XmlChar);
pub type ExternalSubsetSAXFunc          = extern "C" fn(ctx: *c_void, name: *XmlChar, ExternalID: *XmlChar, SystemID: *XmlChar);
pub type GetEntitySAXFunc               = extern "C" fn(ctx: *c_void, name: *XmlChar) -> *c_void /*xmlEntityPtr*/;
pub type GetParameterEntitySAXFunc      = extern "C" fn(ctx: *c_void, name: *XmlChar) -> *c_void /*xmlEntityPtr*/;
pub type EntityDeclSAXFunc              = extern "C" fn(ctx: *c_void, name: *XmlChar, ty: c_int, publicId: *XmlChar, systemId: *XmlChar, content: *XmlChar);
pub type NotationDeclSAXFunc            = extern "C" fn(ctx: *c_void, name: *XmlChar, publicId: *XmlChar, systemId: *XmlChar);
pub type AttributeDeclSAXFunc           = extern "C" fn(ctx: *c_void, elem: *XmlChar, fullname: *XmlChar, ty: c_int, def: c_int, defaultValue: *XmlChar, tree: *c_void /*xmlEnumerationPtr*/);
pub type ElementDeclSAXFunc             = extern "C" fn(ctx: *c_void, name: *XmlChar, ty: c_int, content: *c_void /*xmlElementContentPtr*/);
pub type UnparsedEntityDeclSAXFunc      = extern "C" fn(ctx: *c_void, name: *XmlChar, publicId: *XmlChar, systemId: *XmlChar, notationName: *XmlChar);
pub type SetDocumentLocatorSAXFunc      = extern "C" fn(ctx: *c_void, loc: *c_void /*xmlSAXLocatorPtr*/);
pub type StartDocumentSAXFunc           = extern "C" fn(ctx: *c_void);
pub type EndDocumentSAXFunc             = extern "C" fn(ctx: *c_void);
pub type StartElementSAXFunc            = extern "C" fn(ctx: *c_void, name: *XmlChar, atts: **XmlChar);
pub type EndElementSAXFunc              = extern "C" fn(ctx: *c_void, name: *XmlChar);
pub type ReferenceSAXFunc               = extern "C" fn(ctx: *c_void, name: *XmlChar);
pub type CharactersSAXFunc              = extern "C" fn(ctx: *c_void, ch: *XmlChar, len: c_int);
pub type IgnorableWhitespaceSAXFunc     = extern "C" fn(ctx: *c_void, ch: *XmlChar, len: c_int);
pub type ProcessingInstructionSAXFunc   = extern "C" fn(ctx: *c_void, target: *XmlChar, data: *XmlChar);
pub type CommentSAXFunc                 = extern "C" fn(ctx: *c_void, value: *XmlChar);
pub type CdataBlockSAXFunc              = extern "C" fn(ctx: *c_void, value: *XmlChar, len: c_int);
pub type WarningSAXFunc                 = extern "C" fn(ctx: *c_void /*const char *msg, ...*/);
pub type ErrorSAXFunc                   = extern "C" fn(ctx: *c_void /*const char *msg, ...*/);
pub type FatalErrorSAXFunc              = extern "C" fn(ctx: *c_void /*const char *msg, ...*/);
pub type IsStandaloneSAXFunc            = extern "C" fn(ctx: *c_void) -> int;
pub type HasInternalSubsetSAXFunc       = extern "C" fn(ctx: *c_void) -> int;
pub type HasExternalSubsetSAXFunc       = extern "C" fn(ctx: *c_void) -> int;
pub type StartElementNsSAX2Func         = extern "C" fn(ctx: *c_void, localname: *XmlChar, prefix: *XmlChar, URI: *XmlChar, nb_namespaces: c_int, namespaces: **XmlChar, nb_attributes: c_int, nb_defaulted: c_int, attributes: **XmlChar);
pub type EndElementNsSAX2Func           = extern "C" fn(ctx: *c_void, localname: *XmlChar, prefix: *XmlChar, URI: *XmlChar);
pub type XmlStructuredErrorFunc         = extern "C" fn(userData: *c_void, error: XmlErrorPtr);

pub struct XmlSAXHandler {
    internalSubset:         Option<InternalSubsetSAXFunc>,
    isStandalone:           Option<IsStandaloneSAXFunc>,
    hasInternalSubset:      Option<HasInternalSubsetSAXFunc>,
    hasExternalSubset:      Option<HasExternalSubsetSAXFunc>,
    resolveEntity:          Option<ResolveEntitySAXFunc>,
    getEntity:              Option<GetEntitySAXFunc>,
    entityDecl:             Option<EntityDeclSAXFunc>,
    notationDecl:           Option<NotationDeclSAXFunc>,
    attributeDecl:          Option<AttributeDeclSAXFunc>,
    elementDecl:            Option<ElementDeclSAXFunc>,
    unparsedEntityDecl:     Option<UnparsedEntityDeclSAXFunc>,
    setDocumentLocator:     Option<SetDocumentLocatorSAXFunc>,
    startDocument:          Option<StartDocumentSAXFunc>,
    endDocument:            Option<EndDocumentSAXFunc>,
    startElement:           Option<StartElementSAXFunc>,
    endElement:             Option<EndElementSAXFunc>,
    reference:              Option<ReferenceSAXFunc>,
    characters:             Option<CharactersSAXFunc>,
    ignorableWhitespace:    Option<IgnorableWhitespaceSAXFunc>,
    processingInstruction:  Option<ProcessingInstructionSAXFunc>,
    comment:                Option<CommentSAXFunc>,
    warning:                Option<WarningSAXFunc>,
    error:                  Option<ErrorSAXFunc>,
    fatalError:             Option<FatalErrorSAXFunc>,
    getParameterEntity:     Option<GetParameterEntitySAXFunc>,
    cdataBlock:             Option<CdataBlockSAXFunc>,
    externalSubset:         Option<ExternalSubsetSAXFunc>,
    initialized:            c_uint,
    _private:               *c_void,
    startElementNs:         Option<StartElementNsSAX2Func>,
    endElementNs:           Option<EndElementNsSAX2Func>,
    serror:                 Option<XmlStructuredErrorFunc>,
}

/// Error level type alias
pub type XmlErrorLevel = c_int;

/// No error
pub static XML_ERR_NONE:    XmlErrorLevel = 0;
/// A simple warning
pub static XML_ERR_WARNING: XmlErrorLevel = 1;
/// A recoverable error
pub static XML_ERR_ERROR:   XmlErrorLevel = 2;
/// A fatal error
pub static XML_ERR_FATAL:   XmlErrorLevel = 3;

pub struct XmlError {
    /// What part of the library raised this error
    domain: c_int,
    /// The error code, e.g. an xmlParserError
    code: c_int,
    /// human-readable informative error messag
    message: *c_char,
    /// how consequent is the error
    level: XmlErrorLevel,
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

pub type XmlErrorPtr = *XmlError;

/// libxml2 function bindings
#[link(name = "xml2")]
extern "C" {
    pub fn xmlInitParser();
    pub fn xmlCleanupParser();
    pub fn xmlSAXUserParseMemory(sax: *XmlSAXHandler,
                                 user_data: *c_void,
                                 buffer: *c_char,
                                 size: c_int) -> c_int;
}
