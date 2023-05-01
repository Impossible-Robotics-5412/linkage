#!/usr/bin/env python3

import argparse
from argparse import Namespace
import subprocess
from os import path


def styled_print(message):
    print(f"[🦺 Bob]: {message}")


def linkage_dir():
    return path.dirname(path.abspath(__file__))


def home_dir():
    return path.expanduser("~")


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


def init(no_npm_install):
    if not no_npm_install:
        subprocess.run(["pnpm", "install"])


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


def lint():
    styled_print("Linting entire project...")
    styled_print("Running eslint...")
    subprocess.run(["npx", "eslint", "."])
    styled_print("Running cargo clippy...")
    subprocess.run(["cargo", "clippy"])
    styled_print("Done!")
    exit(0)


def build_cockpit():
    styled_print("Building frontend...")
    subprocess.run(["pnpm", "install"], cwd="cockpit")
    subprocess.run(["pnpm", "run", "tauri", "build"], cwd="cockpit")


def build_carburetor(cargo_path=None, release=False):
    styled_print("Building Carburetor...")
    cargo_build(cargo_path, "carburetor", release=release)


def build(args: Namespace):
    if args.part == "all":
        styled_print("Building all parts...")
        cargo_build(release=args.release)
        build_cockpit()
    elif args.part == "cockpit":
        styled_print("Building cockpit frontend and backend...")
        build_cockpit()
    elif args.part == "carburetor":
        build_carburetor(release=args.release)
    else:
        styled_print("ERROR: Part '{unknown}' not recognized")

    styled_print("Done!")
    exit(0)


def deploy_carburetor(host):
    styled_print("Deploying Carburetor...")
    subprocess.run(["./deploy.sh", host], cwd="carburetor")


def deploy_gauge(host):
    styled_print("Deploying Gauge...")
    subprocess.run(["./deploy.sh", host], cwd="gauge")


def deploy_example(host, example):
    styled_print("Deploying Example...")
    subprocess.run(["./deploy.sh", host, example], cwd="lib/linkage-rs")


def deploy(args: Namespace):
    if args.part == "all":
        styled_print("Deploying all parts...")
        deploy_carburetor(args.host)
        deploy_gauge(args.host)
        deploy_example(args.host, "simple_tankdrive")
    elif args.part == "carburetor":
        deploy_carburetor(args.host)
    elif args.part == "example":
        deploy_example(args.host, "simple_tankdrive")
    elif args.part == "gauge":
        deploy_gauge(args.host)
    else:
        styled_print("ERROR: Part '{unknown}' not recognized")


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

    init_subcommand.add_argument(
        "--no-npm-install",
        help="initialize the project without running npm install",
        action="store_true",
    )

    # Format subcommand
    format_subcommand = subparsers.add_parser("format", help="format all files")

    # Lint subcommand
    lint_subcommand = subparsers.add_parser("lint", help="lints all files")

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
            "gauge",
            "carburetor",
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
        "host",
        help="the host to deploy to",
    )

    deploy_subcommand.add_argument(
        "part",
        help="the part of linkage to deploy",
        choices=["all", "carburetor", "gauge", "example"],
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
            "gauge",
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
        init(no_npm_install=args.no_npm_install)
    elif args.subcommand == "format":
        format()
    elif args.subcommand == "lint":
        lint()
    elif args.subcommand == "build":
        build(args)
    elif args.subcommand == "deploy":
        deploy(args)
    elif args.subcommand == "run":
        run(args)
    else:
        styled_print("ERROR: Unknown subcommond '{args.subcommand}'")
