
# CCP - Clipboard CoPy
*A file copy, horribly written*

## Using

This project exposes two binaries; `ccopy` and `cpaste`. They do the following:

### ccopy
Each argument is interpreted as a filepath and copied verbatim to the "clipboard[1]". All file characters must be unicode compliant and not contain any newline characters (`\n`).

- [1]: the clipboard in this case is a file located under `/tmp/___CCP_COPIED_FILES`, if this file already exists it will get overwritten

### cpaste
Paste the files copied by `ccopy` to whatever directory you specify in the first argument.


*And that's it!*

## Building

Run `nix build`

To run each seperate binary, run: `nix run .#ccopy` or `nix run .#cpaste`. As with any flake, you can substitute the `.` with the url of this repository.

