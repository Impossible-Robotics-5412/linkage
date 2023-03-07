# Linkage

<!--- figlet -f Cyberlarge linkage --->

            _____ __   _ _     _ _______  ______ _______
     |        |   | \  | |____/  |_____| |  ____ |______
     |_____ __|__ |  \_| |    \_ |     | |_____| |______

## Project structure

This repository stores the source directories for the different programs
that for the system. We call the whole project *linkage*.

- [**carburetor**](https://github.com/Impossible-Robotics-5412/linkage/tree/main/carburetor)

  _Carburetor_ is the executable responsible for the actual communication with the motor drivers and other peripherals connected to the Pi.
  It accepts a TCP stream of control instructions and executes these.
  The stream of control instructions is sent by the robot code, which is built on the linkage library (e.g., _linkage-node_).
  On shutdown or termination, Carburetor tries its best to shut down gracefully by putting all motors it controls into a neutral state.
  In normal operation, Carburetor is started up and shut down by Runtime.

- **cockpit**

  Cockpit is the user interface for linkage.
  It is separated in a _backend_ and a _frontend_.
  These can be located on the same host computer or on different devices.
  In case of the web-based _frontend_, the same computer can serve the _frontend_ and _backend_ while the _frontend_ web interface is accessed over the local network.
  The _backend_ and (perhaps multiple different intstances of) _frontend_ communicate over the WebSocket protocol.

    - [**backend**](https://github.com/Impossible-Robotics-5412/linkage/tree/main/cockpit/backend)

      The _cockpit-backend_ is responsible for the communication between the user-facing _cockpit-frontend_ and _runtime_.
      It runs on a computer in the same local network as the robot pilot computer, and the Raspberry Pi embedded in the robot.

    - [**frontend**](https://github.com/Impossible-Robotics-5412/linkage/tree/main/cockpit/web/frontend)/web

      The web _cockpit-frontend_ serves a webpage over the local network through which the robot pilot can control the robot.
      The _frontend_ sends enable and disable instructions to the _backend_ over a WebSocket connection, and the _backend_ sends these to _runtime_ over TCP.
      The _runtime_, in turn enables or disables the robot code built on the _linkage_ library and _carburetor_.

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

- [**runtime**](https://github.com/Impossible-Robotics-5412/linkage/tree/main/runtime)

  The _runtime_ is a daemonized process running on the Raspberry Pi placed on the robot.
  It listens for a TCP stream from the _backend_.
  When a connection to _backend_ has been established, it waits for the signal to start two programs: _carburetor_, and the robot code based on the _linkage_ library.
  After starting these, it listens for a signal to disable these.
  As described above, _carburetor_ can handle a termination signal gracefully, and _runtime_ shuts it down through this method.

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
- [ ] Installation and Usage section (see [#3](https://github.com/Impossible-Robotics-5412/linkage/issues/3))
- [ ] Add examples (see [#2](https://github.com/Impossible-Robotics-5412/linkage/issues/2))
- [X] Unified configuration file (see [#4](https://github.com/Impossible-Robotics-5412/linkage/issues/4))

Thank you for reading about our project and take the time to look at the sky today.
You deserve that.
