# uuid2clip

A basic CLI tool that generates a UUIDv4, puts it in the clipboard, and sends a system notification when done.

The intended use case is to set up a system keyboard shortcut (e.g. `Win+U`) to generate a UUID which can then be
pasted into whatever you're trying to do.

## Setup in Ubuntu

 * Clone and build this with `cargo build`
 * Get the path to the compiled binary (e.g. `/path/to/uuid2clip/target/debug/uuid2clip`)
 * Set up a keyboard shortcut
    * Settings > Keyboard > View and Customise Shortcuts > Custom Shortcuts
    * Add a custom shortcut, e.g. `Generate UUID`
    * Set the command to the compiled binary (e.g. `/path/to/uuid2clip/target/debug/uuid2clip`)
    * Set the Shortcut itself to something you like (I use `Win/Super + U`)
 * Then if I want a UUID, I just hit `Win+U` then `Ctrl+V` to paste it somewhere
