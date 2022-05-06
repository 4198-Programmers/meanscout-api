PREFIX ?= /usr
MANDIR ?= $(PREFIX)/share/man
DOCDIR ?= $(PREFIX)/share/doc/fff

all:
	@echo Run \'make install\' to install meanapi.

install:
	@cargo build
	@mkdir -p $(DESTDIR)$(PREFIX)/bin
	@mkdir -p $(DESTDIR)$(DOCDIR)
	@cp -p /target/debug/meanapi $(DESTDIR)$(PREFIX)/bin/meanapi
	@cp -p Rocket.toml $(DESTDIR)$(PREFIX)/bin/meanapi
	@cp -p README.md $(DESTDIR)$(DOCDIR)
	@chmod 755 $(DESTDIR)$(PREFIX)/bin/meanapi

uninstall:
	@rm -rf $(DESTDIR)$(PREFIX)/bin/meanapi
	@rm -rf $(DESTDIR)$(DOCDIR)