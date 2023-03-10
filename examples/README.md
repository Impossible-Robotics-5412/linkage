# Examples

## Config

- `examples/config/config.local.default.toml`: A default config file for the programs that run on the driver station computer (_cocpit-frontend_, _cockpit-backend_).
- `examples/config/config.pi.default.toml`: A default config file for the programs that run on the Raspberry Pi on the robot. (_runtime_, _carburetor_, robot code based on the _linkage_ library).

These config file should be placed in a directory called `linkage` in the `XDG_CONFIG_HOME` directory. On Linux systems, this means that the programs look for the config file in `~/.config/linkage/config.yml`.

## Linkage-lib

TODO: Write
