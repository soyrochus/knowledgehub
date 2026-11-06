# Ghostty placement rejection correction · 5 November 2026

The user reported `GraphicsTransport: terminal rejected placement` while running
inside Ghostty, where another Kitty graphics application works. This is a failed
native attempt of package `final-04`, not evidence of missing terminal support.
The earlier synthetic checks did not establish successful Ghostty display.

## Cause and bounded correction

The viewer uploaded a PNG with `a=t`, then its combined text/placement reveal sent
`ESC[H ESC[2J` before `a=p`. The full display erase also clears image storage:
Ghostty reclaims unplaced images, so the following placement refers to deleted
data and returns `ENOENT: image not found`. The code discarded that useful
terminal response and showed only the generic placement error.

All viewer/browser text redraws now use home plus ED0 (`ESC[H ESC[0J`), which
clears text without deleting graphics. Images are still deleted only through the
existing owned-ID graphics commands. Negative placement acknowledgements retain
the bounded, sanitized terminal reason and the owned image/placement identities.
No model/fixture/native host/library/codec changes are made.

The corrected unit terminal and compiled PTY responder process terminal controls
in wire order, make ED2/ED3 evict images, reject placement of missing image IDs,
and check that unrelated graphics survive. The unchanged `final-04` package and
new unit regression both reproduce the reported generic error before the fix.
See [old-package-reproduction.json](old-package-reproduction.json) and its
[terminal transcript](old-package-reproduction.ansi). The same unit regression
passes after the correction. The full terminal suite passes 25 tests and 562
assertions against the release host, and all 57 compiled archive commands plus
the existing browser PTY regression pass.

## Primary sources checked

- [Kitty terminal-action interactions](https://sw.kovidgoyal.net/kitty/graphics-protocol/#interaction-with-other-terminal-actions): full-screen erase clears graphics; other text erases do not.
- [Ghostty v1.4.0 display erase](https://github.com/ghostty-org/ghostty/blob/v1.4.0/src/terminal/Terminal.zig): complete erase calls image-storage cleanup; erase below only clears text.
- [Ghostty v1.4.0 image clear](https://github.com/ghostty-org/ghostty/blob/v1.4.0/src/terminal/kitty/graphics_storage.zig): screen clearing also reclaims unplaced images.
- [Ghostty v1.4.0 placement](https://github.com/ghostty-org/ghostty/blob/v1.4.0/src/terminal/kitty/graphics_exec.zig): a missing image ID returns ENOENT.

## Verification and remaining observation

The replacement is `target/terminal-graphics/knowledgehub-terminal-kitty-macos-arm64-final-05.tar.gz`.
[completion.json](completion.json) records its exact identity and 330 checksums.
The 25 compiled graphics PTY cases pass with zero missing-image placements or
uploaded-image erasures and preservation of unrelated graphics in every case.
See [graphics-pty.json](graphics-pty.json), [bun-tests.log](bun-tests.log),
[package-regression.json](package-regression.json), [legacy-pty.json](legacy-pty.json)
and [isolation.json](isolation.json). All 32 semantic/source/reference/receipt
files hashed by the viewer matrix remain unchanged.
Protocol/PTY tests remain simulations; they are not screenshots or execution of
the actual Ghostty renderer. The successful native Kitty/Ghostty checks remain
open. The user must retest the corrected package in their directly connected
Ghostty session; no UI-automation restriction was bypassed. Fixture gaps, source
conflicts, human review, Modelarium C9 and authority boundaries remain unchanged.
