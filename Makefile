CC = g++
#CC = clang++
CFLAGS := -g -Wall -std=c++1y 
LIB := -L./lib
INC := -I./src/include

SRCDIR := ./src
BUILDDIR := ./build
TARGET := ./bin/main.exe
LIBSRC := ./src/lib
LIBBUILD := ./build/lib
LIBTARGET := ./lib/libTOR.a

SRCEXT := cpp
SOURCES := $(wildcard $(SRCDIR)/*.$(SRCEXT))
OBJECTS := $(patsubst $(SRCDIR)/%,$(BUILDDIR)/%,$(SOURCES:.$(SRCEXT)=.o))
LIBSOURCES := $(wildcard $(LIBSRC)/*.$(SRCEXT))
LIBOBJS := $(patsubst $(LIBSRC)/%,$(LIBBUILD)/%,$(LIBSOURCES:.$(SRCEXT)=.o))

#====================================================================
# DRIVERS
#====================================================================
all: $(TARGET)
	$(TARGET)

#====================================================================
# Link and Compile
#====================================================================
$(TARGET): $(OBJECTS) $(LIBTARGET)
	@echo "Linking...";
	$(CC) $(CFLAGS) $^ $(LIB) -o $(TARGET)

$(BUILDDIR)/%.o: $(SRCDIR)/%.$(SRCEXT) 
	@echo "Building...";
	$(CC) $(CFLAGS) $(INC) -c $< -o $@

#====================================================================
# Libraries
#====================================================================
$(LIBTARGET): $(LIBOBJS)
	@echo "Building lib...";
	ar crf $@ $^

$(LIBBUILD)/%.o: $(LIBSOURCES)/%.$(SRCEXT)
	$(CC) $(CFLAGS) $(INC) -c -o $@ $<

#====================================================================
# Cleanup and Settings
#====================================================================

clean:
	@echo "Cleaning executable..."; 
	rm -rf $(TARGET)

wipe:
	@echo "Cleaning all..."; 
	rm -rf $(wildcard $(BUILDDIR)/*.o) $(wildcard $(LIBBUILD)/*.o) $(TARGET) $(LIBTARGET)

.PHONY: clean
