GLSLC=glslangValidator
GLSLCFLAGS=-V

SHD=compute/src/fill.comp.spv\
	graphics/src/test.vert.spv\
	graphics/src/test.frag.spv

all: shaders
.PHONY: all clean shaders

clean:
	$(RM) $(SHD)

shaders: $(SHD)

%.spv: % Makefile
	$(GLSLC) $(GLSLCFLAGS) -o $@ $<

