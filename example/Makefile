CXXFLAGS += -o3 -std=c++0x
LDLIBS = -lutil

.PHONY: all clean distclean

all: pty

pty: pty.o
	$(LINK.cc) $^ $(LDLIBS) -o $@

clean:
	@- $(RM) pty.o pty

distclean: clean
