# TODO

- offer basic API in addtion to embedded graphics
- defer rounding on accumulated quantization error (or use wide enough fixed point)
- decide whether or not to error on error being inconsistent with closest color
- better separate utility stuff (terminal, cga, color cube)

export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"

an error of 0 will have the effect of disabling dithering
dividing error by a scaler will lessen effect of dither
multiplying error will increase effect
driver for display will probably already have an idea of to/from rgb888
for some displays though, this to/from rgb888 can be naive or simplistic
if you want to come up with something better, you have to experiment
that's why giving closure is an option here
