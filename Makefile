all: main

main:
	rustc main.rs

clean:
	rm main

.PHONY: all clean
