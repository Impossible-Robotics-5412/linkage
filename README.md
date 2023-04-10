# Linkage

<!--- figlet -f Cyberlarge linkage --->
```
            _____ __   _ _     _ _______  ______ _______
     |        |   | \  | |____/  |_____| |  ____ |______
     |_____ __|__ |  \_| |    \_ |     | |_____| |______

```

## Getting started

### Installing Linkage on a Pi
1. Clone the repo to the Pi using `git clone https://github.com/Impossible-Robotics-5412/linkage.git`.
2. Navigate into the downloaded repo using `cd linkage`.
3. Run `./install-rpi.sh`. (Please note this might take quite a few minutes as it will install and compile all necessary dependencies!)
4. [Deploy your robot code](#deploy-robot-code).

### Running Cockpit
Currently we don't create prebuilt versions of Cockpit, which means you will have to build it from source.
In the near future this shouldn't be needed anymore for general use.
Please see [Preparing the project](#preparing-the-project) for more information about building and running Cockpit.

## Building Cockpit
### Prerequisites
- [[node](https://nodejs.org/en)] Node is a Javascript runtime.
- [[pnpm](https://pnpm.io)] This project uses pnpm as a node package manager. It is strongly recommended to use this aswel for compatibility!
- [[cargo](https://doc.rust-lang.org/cargo/)] Cargo is the package manager used by Rust. Because this project uses quite some Rust code, this is needed to run most code.
- [[python3](https://www.python.org/downloads/)] Python is used for our build script _bob.py_.

- Follow https://tauri.app/v1/guides/getting-started/prerequisites to install the right dependencies for Tauri.

### Preparing the project
After cloning the repo run `pnpm install`. This will add a config file to `~/.config/linkage/config.toml` which is used for general configuration of the project, and will install the necessary dependencies.

### Running Cockpit
Now you should be able to run `./bob.py run cockpit` from the root of the project to start Cockpit.

## Deploying robot code
To get started, you can deploy the example robot code:
1. Navigate into the example folder using `cd examples/lib/linkage-node`.
2. Run `./deploy.sh`

## Windows support
Currently windows is not supported, but we are looking into this to make sure everyone can enjoy Linkage!

## Project structure

This repository stores the source directories for the different programs
that for the system. We call the whole project *linkage*.

- [**carburetor**](https://github.com/Impossible-Robotics-5412/linkage/tree/main/carburetor)

  _Carburetor_ is the executable responsible for the actual communication with the motor drivers and other peripherals connected to the Pi.
  It accepts a TCP stream of control instructions and executes these.
  The stream of control instructions is sent by the robot code, which is built on the linkage library (e.g., _linkage-node_).
  On shutdown or termination, Carburetor tries its best to shut down gracefully by putting all motors it controls into a neutral state.

- **cockpit**

  Cockpit is the user interface for Linkage.
  It is separated in a _backend_ and a _frontend_.

- [**gauge**](https://github.com/Impossible-Robotics-5412/linkage/tree/main/gauge)

  Gauge is a process that runs on the Pi, that sends system information to Cockpit.
  This includes CPU load, memory usage, running services, etc.

- **lib**

  The _linkage_ library provides a programming interface, which allows people to create programs that control the robot.
  In essense, this whole project revolves around supporting this library to serve as the interface between the pilot station and the operation of the robot.
  It facilitates your 'robot code'.
  We aim for it to be a way for users to create control software that is both

  1. powerful and extensible; while still being
  2. approachable and accessible to those who are learning to write real programs for the first time.

  Because the other programs that are part of this project are built on communication via TCP, the library can be implemented in any number of languages.

    - [**linkage-node**](https://github.com/Impossible-Robotics-5412/linkage/tree/main/lib/linkage-node)

      At this moment, there is one implementation of the _linkage_ library.
      It is written in Node and is also available through npm.

## Contribution

We welcome any and all contributions.

Feel free to open up an issue if you encounter problems or want to talk about adding new features.
Of course, we celebrate contributions in the form of code and documentation as well.
So, pull requests and issues are very welcome.
We will do as much as we can to guide and support new contributors through every step!

Before contributing, be sure to have read our code of conduct.

## Code of Conduct

We ask people who are in any way part of this project to conduct themselves well.
That means that we ask the following:

- Be kind.
- Be inclusive.
- Be respectful.
- Be open to feedback.

If contributors and users perceive that somebody does not act according to this code, we will ask them to change their behaviour.
In case the offence does not end after this, we will take further action, among which is excluding the offender, to ensure the safety and happiness of our contributors.

This code of conduct is subject to elaboration.

## Todo

- [x] Describe project structure in README
- [ ] Introduction section with the following outline to describe the project as a whole (see [#1](https://github.com/Impossible-Robotics-5412/linkage/issues/1))
    - [ ] What?
    - [ ] Why?
    - [ ] How?
- [x] Installation and Usage section (see [#3](https://github.com/Impossible-Robotics-5412/linkage/issues/3))
- [ ] Add examples (see [#2](https://github.com/Impossible-Robotics-5412/linkage/issues/2))
- [X] Unified configuration file (see [#4](https://github.com/Impossible-Robotics-5412/linkage/issues/4))

Thank you for reading about our project and take the time to look at the sky today.
You deserve that.
