# Graphics dependency and source notices

Unmodified direct dependencies: @dagrejs/dagre 3.1.3 (MIT),
@dagrejs/graphlib 4.0.6 (MIT, layout dependency), @resvg/resvg-wasm 2.6.2 (MPL-2.0).
The complete corresponding resvg-js source is available at:
https://github.com/thx/resvg-js/tree/9ca058462ac529120c8cc84ddcd6fef644cc5406
https://codeload.github.com/thx/resvg-js/tar.gz/9ca058462ac529120c8cc84ddcd6fef644cc5406
No modifications are made to the upstream WASM module or its JavaScript binding.
RESVG-LICENSE is the full MPL 2.0 license. The source tree owns its Cargo.lock
and third-party Rust source/dependency declarations. Dagre source revision:
https://github.com/dagrejs/dagre/tree/a73acbfebe6fda008e1a3d80f3738170699c3d6b
Graphlib source: https://github.com/dagrejs/graphlib/tree/v4.0.6

DejaVu Sans 2.37 is unmodified from the official binary release:
https://github.com/dejavu-fonts/dejavu-fonts/releases/tag/version_2_37
https://github.com/dejavu-fonts/dejavu-fonts/releases/download/version_2_37/dejavu-fonts-ttf-2.37.tar.bz2
DEJAVU-LICENSE and DEJAVU-AUTHORS accompany the font. glyph-ranges.json is derived
from this font's Unicode cmap (formats 4 and 12), retaining nonzero glyphs.
It is presentation metadata, not a modified font. No system fonts are loaded.
The executable embeds the font and WASM; these notices need no runtime download.
All asset hashes, package versions and npm integrity values are in inventory.json
and bun.lock. The package copies these notices to graphics-notices/.
