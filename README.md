Reverse Bytes Challenge, completed in Rust.

---

1. Read input from stdin.

2. Reverse the order of input bytes *and* bits (otherwise you'd be rewriting `rev`).

Reversing byte order:

```
start: 11100010 11100010 10111000
end:   10111000 11100010 11100010
```

Reversing bit order:

```
start: 11100010 11100010 10111000
end:   01000111 01000111 00011101
```

Reversing both:

```
start: 11100010 11100010 10111000
end:   00011101 01000111 01000111
```

3. Print to stdout.

Use any language, but stick to the standard library if possible. If you run anything through your program twice, the output should be unchanged, as reversing the data should be, unsurprisingly, reversible.

```
$ echo 'Hello, world!' | your_program
P�&6N��4�66�
$ echo 'Hello, world!' | your_program | your_program
Hello, world!
```
---

Bonus challenges:

1. Accept an optional explicit file argument (stdin should still be default).
2. Make reversing bits and bytes independent and optional (toggled with environment variables, command-line arguments, or both).
3. Reverse every chunk of N bytes, rather than the entire file.
4. Handle files or data that are **20GB+** (stdin should still be default).
