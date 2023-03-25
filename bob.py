#!/usr/bin/env python3

import argparse
import subprocess


def cargo_build(package=None, release=False):
    args = ["cargo", "build"]
    if package:
        args.append(f"--package={package}")
    if release:
        args.append("--release")
    subprocess.run(args)


def build_cockpit_frontend():
    print("👷🏼‍♂️ Bob: Building frontend...")
    subprocess.run(["npm", "install"], cwd="cockpit/frontend/web")
    subprocess.run(["npm", "run", "build"], cwd="cockpit/frontend/web")


def build_cockpit_backend():
    print("👷🏼‍♂️ Bob: Building backend...")
    # TODO: Add ability to use a --release flag for build subcommand.
    cargo_build("cockpit-backend")


def build_runtime():
    print("👷🏼‍♂️ Bob: Building runtime...")
    cargo_build("runtime")


def build_carburetor():
    print("👷🏼‍♂️ Bob: Building carburetor...")
    cargo_build("carburetor")


def build_lib():
    print("👷🏼‍♂️ Bob: Building linkage lib...")
    subprocess.run(["npm", "install"], cwd="lib/linkage-node")
    subprocess.run(["npm", "run", "build"], cwd="lib/linkage-node")


def build_lib_examples():
    print("👷🏼‍♂️ Bob: Building linkage lib examples...")
    subprocess.run(["npm", "link"], cwd="lib/linkage-node")
    subprocess.run(
        ["npm", "link", "@impossiblerobotics/linkage", "--save"],
        cwd="examples/lib/linkage-node",
    )
    subprocess.run(["npm", "run", "build"], cwd="examples/lib/linkage-node")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Build script for the many moving parts of Linkage",
        epilog="Koen Westendorp & Bauke Westendorp, 2023",
    )

    subparsers = parser.add_subparsers(title="subarguments", required=True)
    build = subparsers.add_parser("build", help="build the moving parts of linkage")
    # deploy = subparsers.add_parser("deploy", help="deploy the moving parts of linkage")
    # test = subparsers.add_parser("test", help="test the moving parts of linkage")

    build.add_argument(
        "part",
        help="the part of linkage to build",
        choices=[
            "all",
            "cockpit",
            "cockpit-frontend",
            "cockpit-backend",
            "runtime",
            "carburetor",
            "lib",
            "lib-examples",
            "lib-examples-only",
        ],
    )
    args = parser.parse_args()

    match args.part:
        case "all":
            print("👷🏼‍♂️ Bob: Building all parts...")
            build_cockpit_frontend()
            cargo_build()
            build_lib()
            build_lib_examples()

        case "cockpit":
            print("👷🏼‍♂️ Bob: Building cockpit frontend and backend...")
            build_cockpit_frontend()
            build_cockpit_backend()

        case "cockpit-frontend":
            build_cockpit_frontend()

        case "cockpit-backend":
            build_cockpit_backend()

        case "runtime":
            build_runtime()

        case "carburetor":
            build_carburetor()

        case "lib-examples":
            print("👷🏼‍♂️ Bob: Building linkage lib and its examples...")
            build_lib()
            build_lib_examples()

        case "lib-examples-only":
            print("👷🏼‍♂️ Bob: Building only linkage lib examples...")
            build_lib_examples()

        case "lib":
            build_lib()

        case unknown:
            print("👷🏼‍♂️ Bob: ERROR: Part '{unknown}' not recognized")

    print("👷🏼‍♂️ Bob: Done!")
    exit(0)
