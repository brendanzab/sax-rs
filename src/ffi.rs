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

use std::libc::*;

pub type xmlChar = c_uchar;

// These can be found in libxml/parser.h

/// Special constant found in SAX2 blocks initialized fields
pub static XML_SAX2_MAGIC: c_uint = 0xDEEDBEAF;

pub type resolveEntitySAXFunc           = *u8;  // typedef xmlParserInputPtr (*resolveEntitySAXFunc) (void *ctx, const xmlChar *publicId, const xmlChar *systemId);
pub type internalSubsetSAXFunc          = *u8;  // typedef void (*internalSubsetSAXFunc) (void *ctx, const xmlChar *name, const xmlChar *ExternalID, const xmlChar *SystemID);
pub type externalSubsetSAXFunc          = *u8;  // typedef void (*externalSubsetSAXFunc) (void *ctx, const xmlChar *name, const xmlChar *ExternalID, const xmlChar *SystemID);
pub type getEntitySAXFunc               = *u8;  // typedef xmlEntityPtr (*getEntitySAXFunc) (void *ctx, const xmlChar *name);
pub type getParameterEntitySAXFunc      = *u8;  // typedef xmlEntityPtr (*getParameterEntitySAXFunc) (void *ctx, const xmlChar *name);
pub type entityDeclSAXFunc              = *u8;  // typedef void (*entityDeclSAXFunc) (void *ctx, const xmlChar *name, int type, const xmlChar *publicId, const xmlChar *systemId, xmlChar *content);
pub type notationDeclSAXFunc            = *u8;  // typedef void (*notationDeclSAXFunc)(void *ctx, const xmlChar *name, const xmlChar *publicId, const xmlChar *systemId);
pub type attributeDeclSAXFunc           = *u8;  // typedef void (*attributeDeclSAXFunc)(void *ctx, const xmlChar *elem, const xmlChar *fullname, int type, int def, const xmlChar *defaultValue, xmlEnumerationPtr tree);
pub type elementDeclSAXFunc             = *u8;  // typedef void (*elementDeclSAXFunc)(void *ctx, const xmlChar *name, int type, xmlElementContentPtr content);
pub type unparsedEntityDeclSAXFunc      = *u8;  // typedef void (*unparsedEntityDeclSAXFunc)(void *ctx, const xmlChar *name, const xmlChar *publicId, const xmlChar *systemId, const xmlChar *notationName);
pub type setDocumentLocatorSAXFunc      = *u8;  // typedef void (*setDocumentLocatorSAXFunc) (void *ctx, xmlSAXLocatorPtr loc);
pub type startDocumentSAXFunc           = *u8;  // typedef void (*startDocumentSAXFunc) (void *ctx);
pub type endDocumentSAXFunc             = *u8;  // typedef void (*endDocumentSAXFunc) (void *ctx);
pub type startElementSAXFunc            = *u8;  // typedef void (*startElementSAXFunc) (void *ctx, const xmlChar *name, const xmlChar **atts);
pub type endElementSAXFunc              = *u8;  // typedef void (*endElementSAXFunc) (void *ctx, const xmlChar *name);
pub type referenceSAXFunc               = *u8;  // typedef void (*referenceSAXFunc) (void *ctx, const xmlChar *name);
pub type charactersSAXFunc              = *u8;  // typedef void (*charactersSAXFunc) (void *ctx, const xmlChar *ch, int len);
pub type ignorableWhitespaceSAXFunc     = *u8;  // typedef void (*ignorableWhitespaceSAXFunc) (void *ctx, const xmlChar *ch, int len);
pub type processingInstructionSAXFunc   = *u8;  // typedef void (*processingInstructionSAXFunc) (void *ctx, const xmlChar *target, const xmlChar *data);
pub type commentSAXFunc                 = *u8;  // typedef void (*commentSAXFunc) (void *ctx, const xmlChar *value);
pub type cdataBlockSAXFunc              = *u8;  // typedef void (*cdataBlockSAXFunc) (void *ctx, const xmlChar *value, int len);
pub type warningSAXFunc                 = *u8;  // typedef void (*warningSAXFunc) (void *ctx, const char *msg, ...) LIBXML_ATTR_FORMAT(2,3);
pub type errorSAXFunc                   = *u8;  // typedef void (*errorSAXFunc) (void *ctx, const char *msg, ...) LIBXML_ATTR_FORMAT(2,3);
pub type fatalErrorSAXFunc              = *u8;  // typedef void (*fatalErrorSAXFunc) (void *ctx, const char *msg, ...) LIBXML_ATTR_FORMAT(2,3);
pub type isStandaloneSAXFunc            = *u8;  // typedef int (*isStandaloneSAXFunc) (void *ctx);
pub type hasInternalSubsetSAXFunc       = *u8;  // typedef int (*hasInternalSubsetSAXFunc) (void *ctx);
pub type hasExternalSubsetSAXFunc       = *u8;  // typedef int (*hasExternalSubsetSAXFunc) (void *ctx);
pub type startElementNsSAX2Func         = *u8;  // typedef void (*startElementNsSAX2Func) (void *ctx, const xmlChar *localname, const xmlChar *prefix, const xmlChar *URI, int nb_namespaces, const xmlChar **namespaces, int nb_attributes, int nb_defaulted, const xmlChar **attributes);
pub type endElementNsSAX2Func           = *u8;  // typedef void (*endElementNsSAX2Func)   (void *ctx, const xmlChar *localname, const xmlChar *prefix, const xmlChar *URI);
pub type xmlStructuredErrorFunc         = *u8;  // typedef void (*xmlStructuredErrorFunc) (void *userData, xmlErrorPtr error);

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

/// libxml2 function bindings
#[link_args="-lxml2"]
extern "C" {
    pub fn xmlCleanupParser();
    pub fn xmlSAXUserParseMemory(sax: *xmlSAXHandler,
                                 user_data: *c_void,
                                 buffer: *c_char,
                                 size: c_int) -> c_int;
}
