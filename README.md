# Learn Bevy's ECS by ripping off somebody else's project (Code)

Repository of the code for the mini-book [Learn Bevy's ECS by ripping off someone else's project](https://saveriomiroddi.github.io/learn_bevy_ecs_by_ripping_off).

![Rusty Roguelike](/images/readme/rusty_roguelike.png?raw=true)

## Notes

In the steps from 10.x to 15.01 (that is, except the last), the FOV flickers. The fix (see [fix commit](/../../commit/71655f2d7e)) can be easily backported to the previous steps; if anybody wants to contribute the backport, they're very welcome 😄.

## Structure

Both the source and destination projects have one directory (workspace) for each step; the port has a shared `target` directory (in the parent directory of the workspaces), so that one can switch across projects, and recycle (where possible) the compilation results.

The `util` directory contains legacy scripts, which I've used while developing this port, as part of the [Rust Game Ports project](https://github.com/rust-gamedev/rust-game-ports).
