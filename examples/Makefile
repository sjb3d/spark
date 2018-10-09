GLSLC=glslangValidator
GLSLCFLAGS=-V

SHD=src/bin/compute_fill.comp.spv

all: shaders
.PHONY: all clean shaders

clean:
	$(RM) $(SHD)

shaders: $(SHD)

%.spv: % Makefile
	$(GLSLC) $(GLSLCFLAGS) -o $@ $<

