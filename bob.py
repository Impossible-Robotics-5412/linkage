#!/usr/bin/env python3

import argparse
from argparse import Namespace
import subprocess
import platform
import shutil
from os import makedirs, path, environ


def styled_print(message):
    print(f"[👷🏼‍♂️ Bob]: {message}")


def linkage_dir():
    return path.dirname(path.abspath(__file__))


def home_dir():
    return path.expanduser("~")


def is_raspberry_pi():
    return platform.machine() == "armv7l"


def cargo_build(cargo_path=None, package=None, release=False):
    cargo = "cargo" if not cargo_path else cargo_path

    args = [cargo, "build"]
    if package:
        args.append(f"--package={package}")
    if release:
        args.append("--release")

    subprocess.run(args, cwd=linkage_dir())


def cargo_run(cargo_path=None, package=None, release=False):
    cargo = "cargo" if not cargo_path else cargo_path

    args = [cargo, "run"]
    if package:
        args.append(f"--package={package}")
    if release:
        args.append("--release")

    subprocess.run(args, cwd=linkage_dir())


def init():
    subprocess.run(["pnpm", "install"])
    create_config_file()


def format():
    styled_print("Formatting entire project...")
    styled_print("Running prettier...")
    subprocess.run(
        [
            "npx",
            "prettier",
            "--write",
            "--config",
            ".prettierrc.json",
            "{**/*,*}.{js,ts,json,css,scss,sass,html,d.ts,svelte}",
        ]
    )
    styled_print("Running black...")
    subprocess.run(["black", "."])
    styled_print("Running rustfmt...")
    subprocess.run(["cargo", "fmt"])
    styled_print("Done!")
    exit(0)


def build_cockpit():
    styled_print("Building frontend...")
    subprocess.run(["pnpm", "install"], cwd="cockpit")
    subprocess.run(["pnpm", "run", "tauri", "build"], cwd="cockpit")


def build_carburetor(cargo_path=None, release=False):
    styled_print("Building Carburetor...")
    cargo_build(cargo_path, "carburetor", release=release)


def build_lib():
    styled_print("Building Linkage-lib...")
    subprocess.run(["pnpm", "run", "build"], cwd="lib/linkage-node")


def build_lib_example():
    styled_print("Building Linkage-lib example...")
    # FIXME: We need to use sudo here because of permission issues, but we should try to find a workaround for this...
    subprocess.run(["pnpm", "link"], cwd="lib/linkage-node")
    subprocess.run(
        ["pnpm", "link", "@impossiblerobotics/linkage", "--save"],
        cwd="examples/lib/linkage-node",
    )
    subprocess.run(["pnpm", "run", "build"], cwd="examples/lib/linkage-node")


def build(args: Namespace):
    if args.part == "all":
        styled_print("Building all parts...")
        cargo_build(release=args.release)
        build_lib()
        build_lib_example()
        build_cockpit()
    elif args.part == "cockpit":
        styled_print("Building cockpit frontend and backend...")
        build_cockpit()
    elif args.part == "carburetor":
        build_carburetor(release=args.release)
    elif args.part == "lib-example":
        styled_print("Building Linkage-lib and its example...")
        build_lib()
        build_lib_example()
    elif args.part == "lib-example-only":
        styled_print("Building only Linkage-lib example...")
        build_lib_example()
    elif args.part == "lib":
        build_lib()
    else:
        styled_print("ERROR: Part '{unknown}' not recognized")

    styled_print("Done!")
    exit(0)


def deploy_carburetor():
    styled_print("Deploying Carburetor...")
    subprocess.run(["./deploy.sh"], cwd="carburetor")


def deploy_example():
    styled_print("Deploying Linkage-lib example...")
    subprocess.run(["./deploy.sh"], cwd="examples/lib/linkage-node")


def deploy(args: Namespace):
    if args.part == "all":
        styled_print("Deploying all parts...")
        deploy_carburetor()
    elif args.part == "carburetor":
        deploy_carburetor()
    elif args.part == "lib-example":
        deploy_example()
    else:
        styled_print("ERROR: Part '{unknown}' not recognized")


def create_config_file():
    config_folder_path = f"{home_dir()}/.config/linkage"
    config_file_path = f"{config_folder_path}/config.toml"

    example_config_file_path = f"{linkage_dir()}/examples/config/config.default.toml"
    if path.isfile(config_file_path):
        styled_print(f"Config file already exists at '{config_file_path}'.")
    else:
        styled_print(f"Creating default config file at '{config_file_path}'.")
        makedirs(config_folder_path, exist_ok=True)
        shutil.copy(
            src=example_config_file_path,
            dst=config_file_path,
        )


def install_node_js():
    styled_print("Installing NodeJS")
    curl = subprocess.Popen(
        [
            "curl",
            "-fsSL",
            "https://deb.nodesource.com/setup_lts.x",
        ],
        stdout=subprocess.PIPE,
    )
    subprocess.run(
        [
            "sudo",
            "-E",
            "bash",
            "-",
        ],
        stdin=curl.stdout,
    )

    subprocess.run(["sudo", "apt-get", "install", "-y", "nodejs"])


def install_rust():
    styled_print("Installing Rust")
    curl = subprocess.Popen(
        [
            "curl",
            "https://sh.rustup.rs",
            "-sSf",
        ],
        stdout=subprocess.PIPE,
    )
    subprocess.run(
        ["sh"],
        stdin=curl.stdout,
    )


def install_libudev():
    styled_print("Installing libudev")
    subprocess.run(["sudo", "apt-get", "install", "-y", "libudev-dev"])


def install():
    node_path = "/usr/bin/node"
    cargo_path = f"{home_dir()}/.cargo/bin/cargo"

    if not is_raspberry_pi():
        print("You should only run this command on a Raspberry Pi!")
        exit(1)

    create_config_file()

    if not path.isfile(node_path):
        install_node_js()
    styled_print("NodeJS is installed")

    if not path.isfile(cargo_path):
        install_rust()
    styled_print("Rust is installed")

    # FIXME: Check if libudev has been installed already. (maybe use `pkg-config --modversion udev`)
    #        libudev is needed for getting the gamepad input. That means libudev only is needed when you
    #        run cockpit-backend on the pi.
    install_libudev()
    styled_print("Libudev is installed")

    build_carburetor(cargo_path, release=True)

    styled_print("Done")


def run_carburetor(release=False, no_build=False):
    if not no_build:
        build_carburetor(release=release)
    styled_print("Running Carburetor")
    cargo_run(package="carburetor", release=release)


def run_cockpit(release=False):
    if release:
        build_cockpit()
    else:
        styled_print("Running Cockpit")
        subprocess.run(["pnpm", "run", "tauri", "dev"], cwd="cockpit")


def run(args: Namespace):
    if args.part == "carburetor":
        run_carburetor(release=args.release, no_build=args.no_build)
    elif args.part == "cockpit":
        run_cockpit(release=args.release)
    else:
        styled_print("ERROR: Part '{unknown}' not recognized")

    styled_print("Done!")
    exit(0)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Manager script for the many moving parts of Linkage",
        epilog="Koen Westendorp & Bauke Westendorp, 2023",
    )

    subparsers = parser.add_subparsers(
        title="subarguments", dest="subcommand", required=True
    )

    # Init subcommand
    init_subcommand = subparsers.add_parser("init", help="initializes the project")

    # Format subcommand
    format_subcommand = subparsers.add_parser("format", help="format all files")

    # Build subcommand
    build_subcommand = subparsers.add_parser(
        "build", help="build the moving parts of linkage"
    )

    build_subcommand.add_argument(
        "part",
        help="the part of linkage to build",
        choices=[
            "all",
            "cockpit",
            "carburetor",
            "lib",
            "lib-example",
            "lib-example-only",
        ],
    )

    build_subcommand.add_argument(
        "--release",
        "-r",
        help="compile rust binaries in release mode",
        action="store_true",
    )

    # Deploy subcommand
    deploy_subcommand = subparsers.add_parser(
        "deploy",
        help="deploy the moving parts of linkage",
    )

    deploy_subcommand.add_argument(
        "part",
        help="the part of linkage to deploy",
        choices=["all", "carburetor", "lib-example"],
    )

    # Install subcommand
    install_subcommand = subparsers.add_parser(
        "install",
        help="Run this command on a Raspberry Pi to install all the moving parts of linkage to make it ready for deploying Linkage-lib programs.",
    )

    # Run subcommand
    run_subcommand = subparsers.add_parser(
        "run",
        help="run moving parts of linkage",
    )

    run_subcommand.add_argument(
        "part",
        help="the part of linkage to run",
        choices=[
            "cockpit",
            "carburetor",
        ],
    )

    run_subcommand.add_argument(
        "--release",
        "-r",
        help="compile rust binaries in release mode",
        action="store_true",
    )

    run_subcommand.add_argument(
        "--no-build",
        help="don't build part before running",
        action="store_true",
    )

    # Parsing
    args = parser.parse_args()

    if args.subcommand == "init":
        init()
    elif args.subcommand == "format":
        format()
    elif args.subcommand == "build":
        build(args)
    elif args.subcommand == "deploy":
        deploy(args)
    elif args.subcommand == "install":
        install()
    elif args.subcommand == "run":
        run(args)
    else:
        styled_print("ERROR: Unknown subcommond '{args.subcommand}'")
