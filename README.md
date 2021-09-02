# No nonsense clip
A simple clipboard program

## Why
Clipboards on various platforms are a bit annoying to use.
While you can

    cat something.txt | clip.exe

on windows, you can't

    clip.exe | something.txt

since the windows clipboard does not support it.

Similarly, on linux

    cat something.txt | xclip -selection clipboard

is just a bit cumbersome to type.

I wanted a simple, cross-platform solution, which this is (hopefully).

## Usage
    cat something.txt | nnc
    nnc | something_else.txt
