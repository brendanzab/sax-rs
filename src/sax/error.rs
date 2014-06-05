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

//! Error handling

use std::fmt;
use std::str::raw::from_c_str;

use super::ffi;

/// The severity of the error
#[deriving(Clone, Eq, Show)]
pub enum ErrorLevel {
    /// A simple warning
    Warning,
    /// A recoverable error
    Error,
    /// A fatal error
    Fatal,
}

impl ErrorLevel {
    fn from_constant(value: ffi::xmlErrorLevel) -> Option<ErrorLevel> {
        match value {
            ffi::XML_ERR_WARNING => Some(Warning),
            ffi::XML_ERR_ERROR   => Some(Error),
            ffi::XML_ERR_FATAL   => Some(Fatal),
            _                    => None,
        }
    }
}

/// An XML parse error
#[deriving(Clone, Eq)]
pub struct ErrorData {
    level: ErrorLevel,
    line: uint,
    column: uint,
    message: String,
}

impl ErrorData {
    pub unsafe fn from_ptr(error: *ffi::xmlError) -> Option<ErrorData> {
        ErrorLevel::from_constant((*error).level).map(|level| {
            ErrorData {
                level:      level,
                message:    from_c_str((*error).message),
                line:       (*error).line as uint,
                column:     (*error).int2 as uint,
            }
        })
    }
}

impl fmt::Show for ErrorData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}:{} {}: {}",
               self.line,
               self.column,
               self.level,
               self.message)
    }
}
