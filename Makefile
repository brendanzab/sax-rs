# Copyright 2014 The SAX-RS Developers. For a full listing of the authors,
# refer to the AUTHORS file at the top-level directory of this distribution.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

RUSTC               = rustc
RUSTDOC             = rustdoc

SRC_DIR             = src
TEST_DIR            = test
LIB_FILE            = $(SRC_DIR)/sax/lib.rs

CRATE_NAME          = $(shell $(RUSTC) --crate-name $(LIB_FILE))
CRATE_FILES         = $(shell $(RUSTC) --crate-file-name $(LIB_FILE))

DOC_DIR             = doc
LIB_DIR             = lib

INSTALL_PREFIX      = /usr/local
LIB_INSTALL_DIR     = $(INSTALL_PREFIX)/lib

all: lib doc

lib:
	mkdir -p $(LIB_DIR)
	$(RUSTC) --out-dir=$(LIB_DIR) -O $(LIB_FILE)

check:
	mkdir -p $(TEST_DIR)
	$(RUSTC) --out-dir=$(TEST_DIR) --test $(LIB_FILE)
	$(TEST_DIR)/sax

doc:
	mkdir -p $(DOC_DIR)
	$(RUSTDOC) -o $(DOC_DIR) $(LIB_FILE)

install: lib
	@ $(foreach crate, $(CRATE_FILES), \
		cp $(LIB_DIR)/$(crate) $(LIB_INSTALL_DIR)/$(crate) && \
		echo "Installed $(crate) to $(LIB_INSTALL_DIR)" ; \
	)

uninstall:
	@-rm $(LIB_INSTALL_DIR)/lib$(CRATE_NAME)-*.rlib ||:

clean:
	rm -rf $(LIB_DIR)
	rm -rf $(TEST_DIR)
	rm -rf $(DOC_DIR)

.PHONY: \
	all \
	lib \
	check \
	doc \
	install \
	uninstall \
	clean
