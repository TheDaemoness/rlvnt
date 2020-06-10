# rlvnt
__A tool for extracting the broadly-defined "relevant" parts of logs.__

**rlvnt** is a tool to extract ranges of text from text logs.
It provides more general options for marking the
beginnings and ends of ranges than your usual
`/pattern/,/pattern/` awk operation,
including operations based on *the last line containing a match*.

It does this while remaining reasonably memory-light.
The entire input will be buffered into memory only in the worst case.
