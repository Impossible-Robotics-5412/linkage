Operate motor controllers using PWM pins from a Raspberry Pi. This program
receives messages on a TCP stream and applies the requested instructions to
motor controllers and in the future external peripherals attached to the Pi.

<!--- figlet -f rounded carburetor --->

```
                   _
                  | |                        _
  ____ _____  ____| |__  _   _  ____ _____ _| |_ ___   ____
 / ___|____ |/ ___)  _ \| | | |/ ___) ___ (_   _) _ \ / ___)
( (___/ ___ | |   | |_) ) |_| | |   | ____| | || |_| | |
 \____)_____|_|   |____/|____/|_|   |_____)  \__)___/|_|

             By Koen & Bauke Westendorp, 2023.
```

This program allows people of all skill levels to control
robots built for educational purposes. It provides an abstraction over the
control peripherals that can be accessed through TCP. It is
designed to be run as a daemon.

_WARNING: This project is in a highly unstable and experimental stage. It cannot
be relied upon. The security and safety of the operation of this program cannot
be guaranteed. Do not use this software._

## Configuration

No user-friendly configuration is set up, yet. This will be added in the future.

The configuration will include some way of specifying appropriate PWM
frequencies and GPIO pins for interfacing with peripherals such as sensors.

## Usage

### Interface

#### TCP

Default port: `48862`.

Because the TCP interface is meant for inter-process communication between some
user-friendly API and the motor controller and sensors, the port should not be
exposed to the network.

#### Protocol

Request messages are 8 bytes (64 bits) long, and are layed out as follows:

| byte | purpose                                    |
| :--: | :----------------------------------------- |
|  0   | Instruction                                |
|      | - 0: control motor                         |
|      | - ...: future control instructions     |
|  1   | Channel (in case of control instructions)  |
|      | - 0: Pwm0                                  |
|      | - 1: Pwm1                                  |
|      | - ...: future additions                    |
|  2   | Empty (possible future applications)       |
|  3   | Empty                                      |
|  4   | In case of control instructions, the fol-  |
| ...  | lowing 4 bytes represent a big-endian f32. |
|  7   | Otherwise Empty like 2-3.                  |

##### Examples

###### Set PWM pin 1 to 50% forward speed

```
[0x00, 0x01, 0x00, 0x00, 0x3f, 0x00, 0x00, 0x00]
  |     |    ----------  ----------------------
 instr chan    empty       f32 with value 0.5
```

### Behavior

When terminated (through SIGINT by Ctrl-C or through SIGTERM by
`pkill carburetor`), the program will set all motors to neutral before exiting.
This is does not occur when the program is killed by an actual SIGKILL. This
means of termination can thus pose a danger of leaving the motors running until
the program is restarted.

**Thus, terminate the program using SIGINT or SIGTERM, and _never_ SIGKILL.**

**Use SIGKILL iff the operating environment is absolutely safe and it is
absolutely necessary.**

### Daemon

The program is intended to be run as a standalone layer between your own TCP packets and the motor controllers.

<details>
<summary>How to run <em>carburetor</em> using systemd</summary>
To run as daemon using systemd, start it:

```console
systemctl start carburetor.service
```

If you wish to enable it on startup, enable it:

```console
systemctl enable carburetor.service
```

#### Restarting

Currently, the restarting policy is set to 3 seconds.

#### Logs

The logs of the daemonized process can be inspected using:

```console
journalctl -xeu carburetor.service
```
</details>

## Installation

### Prerequisites

The following information assumes you have [Git](https://git-scm.com/) and a
[Rust](https://rust-lang.org/) toolchain installed. Remote deployment relies on
[`cargo-cross`](https://github.com/cross-rs/cross) for cross-compilation.
`cross` builds the binary for the Pi in a Docker container.

### On Raspberry Pi

To install this program on a system directly, clone this repository into a
suitable location. Go into this directory, and run:

```console
cargo build --release
install target/release/carburetor /usr/bin/carburetor
# Optionally install the service.
install -Dm644 carburetor.service /etc/systemd/system/carburetor.service
sudo systemctl daemon-reload
```

If you wish, the program can now be run by invoking the command `carburetor`.

(See [Daemon](#daemon) for starting or enabling the service.)

### Remotely

During development or for convenience, the `deploy.sh` file allows you to
locally cross-compile the executable, and deploy the program to the Pi remotely.

You can run `deploy-systemd.sh` to cross-compile, deploy the
binary, _and_ install the systemd service. It also restart the daemon with the
new binary.