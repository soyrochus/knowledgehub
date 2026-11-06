# Remaining native-terminal observations

Tasks 5.4 (Kitty) and 5.5 (Ghostty) remain open. Use the archive identified in
completion.json, unpack it into a fresh local APFS directory and retain its
SHA-256. Keep kh and knowledgehub-client-host together. Record macOS/architecture,
terminal version, package identity and each exact content selection/anchor.
Use a directly connected terminal; tmux/screen are outside this viewer.

For each terminal independently, run:

```sh
./kh view model:requirement:ack
./kh view model:journey:change --projection journey
./kh view model:lifecycle:offer --projection lifecycle
./kh --model equipment view model:entity:Booking
```

Observe readable labels, types, directions and selected markers. Use `n`/`p`,
`e`, Enter and `s` to inspect exact nodes, parallel assertions, native owners,
source revisions/anchors and an explicit excerpt. For journey/lifecycle, inspect
recorded list order and transitions/guards; retain proposed-expiry qualifications.
Use `0` for an overview, `+`/`-` and arrows to read clipped portions. Verify help,
focus/inspector scrolling, small-window notice, resize recovery and Esc/q cleanup.
Capture screenshots with exact fixture selections/anchors and observed limitations.
Check the cursor and shell input after normal close and handled cancellation.

From `./kh browse`, select R8, retain page/filter/selection and press uppercase V.
Close after inspection and confirm the same browser state. Lowercase v still
exports explicitly. No file or browser should open merely by invoking graphics.

In an actual terminal lacking the required graphics/geometry subset, record its
version and the unavailable reason, exit 6 and usable text/HTML alternatives.
This actual fallback observation is still open, separately from synthetic tests.

An observer must record what was seen and which requirements remain unverified.
Actual terminal execution does not establish human meaning/usability acceptance,
resolve FruitTrucks policy/source conflicts or transfer model authority.
