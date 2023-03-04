Operate motor controllers using PWM pins from a Raspberry Pi. This program
receives messages on a TCP stream and applies the requested instructions to
motor controllers and can query the status of the system and in the future
external peripherals attached to the Pi.

```
                   _
                  | |                        _
  ____ _____  ____| |__  _   _  ____ _____ _| |_ ___   ____
 / ___|____ |/ ___)  _ \| | | |/ ___|____ (_   _) _ \ / ___)
( (___/ ___ | |   | |_) ) |_| | |   / ___ | | || |_| | |
 \____)_____|_|   |____/|____/|_|   \_____|  \__)___/|_|

             By Koen & Bauke Westendorp, 2023.
```

This program serves as what can be described as a raw backend to
[_runtime_](https://github.com/Impossible-Robotics-5412/linkage/tree/main/runtime).
_runtime_ interacts starts _carburetor_ and the robot code written with the
_linkage_ library. This library allows people of all skill levels to control
robots built for educational purposes. It provides an abstraction over the
control and query of peripherals that can be accessed through TCP. It is
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
|      | - ...: additional control instructions     |
|      | - 100: query battery                       |
|      | - 101: query memory                        |
|      | - 102: query cpu                           |
|      | - ...: additional query instructions       |
|  1   | Channel (in case of control instructions)  |
|      | - 0: Pwm0                                  |
|      | - 1: Pwm1                                  |
|      | - ...: future additions                    |
|  2   | Empty (possible future applications)       |
|  3   | Empty                                      |
|  4   | In case of control instructions, the fol-  |
| ...  | lowing 4 bytes represent a big-endian f32. |
|  7   | Otherwise Empty like 2-3.                  |

##### Control

When a message with a query is received, the appropriate response will be sent
back over the TCP stream.

##### Query

When a message with a query is received, the appropriate response will be sent
back over the TCP stream. Currently, these responses take the shape of utf-8
text describing the status of the battery (mock), memory, or cpu. In the future,
these responses might take the shape of messages similar to the requests.

##### Examples

###### Set PWM pin 1 to 50% forward speed

```
[0x00, 0x01, 0x00, 0x00, 0x3f, 0x00, 0x00, 0x00]
  |     |    ----------  ----------------------
 instr chan    empty       f32 with value 0.5
```

###### Query memory usage of the Pi

```
[0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
  |    ----------------------------------------
 instr                  empty
```

This will (currently) return something like:

```
Memory: 26% (242376 / 944268)
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

The program is intended to be run as a child process invoked by [_runtime_](https://github.com/Impossible-Robotics-5412/linkage/tree/main/runtime).
This is not _necessary_, though.
In case you want to use _carburetor_ as a standalone layer between your own TCP packets and the motor controllers, you might want to run it as a daemon.
In that use case, we want it to start on boot, and to restart if anything has gone wrong.

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
# (not necessary when you run carburetor in conjunction with runtime)
install -Dm644 carburetor.service /etc/systemd/system/carburetor.service
sudo systemctl daemon-reload
```

If you wish, the program can now be run by invoking the command `carburetor`.

(See [Daemon](#daemon) for starting or enabling the service.)

### Remotely

During development or for convenience, the `deploy.sh` file allows you to
locally cross-compile the executable, and deploy the program to the Pi remotely.

Change the parameter values in the `deploy.sh` script if necessary. When
configured appropriately, run it with `./deploy.sh`.


<details>
<summary>
<h4>Daemonized</h4>
</summary>

If necessary, you can run `deploy-systemd.sh` to cross-compile, deploy the
binary, _and_ install the systemd service. It also restart the daemon with the
new binary. However, in the way the project is used at this moment, this is no
longer necessary, since _carburetor_ is spawned as a child process of _runtime_
together with the robot code entrypoint. That means that the whole lifetime of
_carburetor_ is managed from _runtime_, and we have no need for daemonized
operation.
</details>
