#!/usr/bin/env python3

import argparse
from argparse import Namespace
import subprocess


def styled_print(message):
    print(f"[👷🏼‍♂️ Bob]: {message}")


def cargo_build(package=None, release=False):
    args = ["cargo", "build"]
    if package:
        args.append(f"--package={package}")
    if release:
        args.append("--release")
    subprocess.run(args)


def format():
    styled_print("Formatting entire project...")
    styled_print("Running prettier...")
    subprocess.run(["npx", "prettier", "-w", "--config", ".prettierrc", "."])
    styled_print("Running black...")
    subprocess.run(["black", "."])
    styled_print("Running rustfmt...")
    subprocess.run(["cargo", "fmt"])
    styled_print("Done!")
    exit(0)


def build_cockpit_frontend():
    styled_print("Building frontend...")
    subprocess.run(["npm", "install"], cwd="cockpit/frontend/web")
    subprocess.run(["npm", "run", "build"], cwd="cockpit/frontend/web")


def build_cockpit_backend(release=False):
    styled_print("Building backend...")
    # TODO: Add ability to use a --release flag for build subcommand.
    cargo_build("cockpit-backend", release=release)


def build_runtime(release=False):
    styled_print("Building runtime...")
    cargo_build("runtime", release=release)


def build_carburetor(release=False):
    styled_print("Building carburetor...")
    cargo_build("carburetor", release=release)


def build_lib():
    styled_print("Building linkage lib...")
    subprocess.run(["npm", "install"], cwd="lib/linkage-node")
    subprocess.run(["npm", "run", "build"], cwd="lib/linkage-node")


def build_lib_example():
    styled_print("Building linkage lib example...")
    # FIXME: We need to use sudo here because of permission issues, but we should try to find a workaround for this...
    subprocess.run(["sudo", "npm", "link"], cwd="lib/linkage-node")
    subprocess.run(
        ["sudo", "npm", "link", "@impossiblerobotics/linkage", "--save"],
        cwd="examples/lib/linkage-node",
    )
    subprocess.run(["npm", "run", "build"], cwd="examples/lib/linkage-node")


def build(args: Namespace):
    if args.part == "all":
        styled_print("Building all parts...")
        cargo_build(release=args.release)
        build_lib()
        build_lib_example()
        build_cockpit_frontend()
    elif args.part == "cockpit":
        styled_print("Building cockpit frontend and backend...")
        build_cockpit_frontend()
        build_cockpit_backend(release=args.release)
    elif args.part == "cockpit-frontend":
        build_cockpit_frontend()
    elif args.part == "cockpit-backend":
        build_cockpit_backend(release=args.release)
    elif args.part == "runtime":
        build_runtime(release=args.release)
    elif args.part == "carburetor":
        build_carburetor(release=args.release)
    elif args.part == "lib-example":
        styled_print("Building linkage lib and its example...")
        build_lib()
        build_lib_example()
    elif args.part == "lib-example-only":
        styled_print("Building only linkage lib example...")
        build_lib_example()
    elif args.part == "lib":
        build_lib()
    else:
        styled_print("ERROR: Part '{unknown}' not recognized")

    styled_print("Done!")
    exit(0)


def deploy_runtime():
    styled_print("Deploying runtime...")
    subprocess.run(["./deploy.sh"], cwd="runtime")


def deploy_carburetor():
    styled_print("Deploying carburetor...")
    subprocess.run(["./deploy.sh"], cwd="carburetor")


def deploy_example():
    styled_print("Deploying example...")
    subprocess.run(["./deploy.sh"], cwd="examples/lib/linkage-node")


def deploy(args: Namespace):
    if args.part == "all":
        styled_print("Deploying all parts...")
        deploy_runtime()
        deploy_carburetor()
    elif args.part == "runtime":
        deploy_runtime()
    elif args.part == "carburetor":
        deploy_carburetor()
    elif args.part == "lib-example":
        deploy_example()
    else:
        styled_print("ERROR: Part '{unknown}' not recognized")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Manager script for the many moving parts of Linkage",
        epilog="Koen Westendorp & Bauke Westendorp, 2023",
    )

    subparsers = parser.add_subparsers(
        title="subarguments", dest="subcommand", required=True
    )

    format_subcommand = subparsers.add_parser("format", help="format all files")

    build_subcommand = subparsers.add_parser(
        "build", help="build the moving parts of linkage"
    )

    build_subcommand.add_argument(
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

    deploy_subcommand = subparsers.add_parser(
        "deploy",
        help="deploy the moving parts of linkage",
    )

    deploy_subcommand.add_argument(
        "part",
        help="the part of linkage to deploy",
        choices=["all", "runtime", "carburetor", "lib-example"],
    )

    args = parser.parse_args()

    if args.subcommand == "format":
        format()
    elif args.subcommand == "build":
        build(args)
    elif args.subcommand == "deploy":
        deploy(args)
    else:
        styled_print("ERROR: Unknown subcommond '{args.subcommand}'")
