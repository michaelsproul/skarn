Skarn | Solid File Sync
====

Skarn is a file copying tool with the following features:

* Selective copying, via simple lists of paths and patterns (similar to `.gitignore`).
* Robust symlink support (either copied verbatim, rewritten, or followed).
* Memory safety, thanks to the beautiful [Rust Programming Language][rust].

It aims to be useful for the following things:

* System backups.
* Media library sync, similar to iTunes' selective copying.
* Backing up collections of Git/Mercurial repositories without copying (potentially large) ignored files.
* Personal file sync ala Dropbox.

Possible Additions
====

* Support for mapping a function (shell command) over the tree of files copied, like `xargs`.
  This would allow songs to be converted to a lower bitrate during music player sync.
* Efficient moves, via hashing and re-structuring ala Git and Jesse Kornblum's [hashdeep][hashdeep].
  Not sure if the memory requirements for this would be too insane.

[rust]: https://rust-lang.org/
[hashdeep]: https://github.com/jessek/hashdeep/

License
====

All Skarn implementation code is licensed under the terms of the GPLv3 or higher.

See http://www.gnu.org/copyleft/gpl.html
