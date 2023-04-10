# Cockpit

Cockpit is the user interface for Linkage.
It is separated in a _backend_ and a _frontend_ written in Rust and Svelte respectively.

## Development

First of all, follow https://tauri.app/v1/guides/getting-started/prerequisites to install the right dependencies for Tauri.

Once you have done this, start the local development server by running `./bob.py run cockpit` in the root of the project.

## Building

You can build a release version of Cockpit running `./bob.py build cockpit` in the root of the project. This will build the project and open an installer for you to install the program onto your system.