GLSLC=glslangValidator
GLSLCFLAGS=-V

SHD=compute-lib/src/fill.comp.spv\
	graphics-lib/src/test.vert.spv\
	graphics-lib/src/test.frag.spv

all: shaders
.PHONY: all clean shaders

clean:
	$(RM) $(SHD)

shaders: $(SHD)

%.spv: % Makefile
	$(GLSLC) $(GLSLCFLAGS) -o $@ $<

