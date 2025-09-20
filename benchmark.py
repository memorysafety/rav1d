#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "plumbum",
#     "typer",
# ]
# ///

import json
from pathlib import Path
from typing import Annotated
from urllib.parse import urlparse
from urllib.request import urlretrieve
import typer
from plumbum import local
from plumbum.commands.base import BoundCommand
from plumbum.machines.local import LocalCommand
from typer import Argument, Option

def run(cmd: LocalCommand | BoundCommand):
    print(cmd)
    cmd()

def host_target(rustc: LocalCommand) -> str:
    output: str = rustc["-vV"]()
    prefix = "host: "
    for line in output.split("\n"):
        if line.startswith(prefix):
            return line[len(prefix):]
    raise RuntimeError("rustc host target not found")

def main(
    threads: Annotated[list[int], Option(help="list of number of threads to test with")],
    cache: Annotated[bool, Option(help="cache results")] = False,
):
    rustc = local["rustc"]
    cargo = local["cargo"]
    meson = local["meson"]
    ninja = local["ninja"]
    hyperfine = local["hyperfine"]

    dir = Path("benchmarks")

    video_url = "http://download.opencontent.netflix.com.s3.amazonaws.com/AV1/Chimera/Old/Chimera-AV1-8bit-1280x720-3363kbps.ivf"
    video_path = Path(urlparse(video_url).path)
    video_path = dir / video_path.name

    dir.mkdir(exist_ok=True)
    if not video_path.exists():
        urlretrieve(video_url, video_path)
    
    target = host_target(rustc)

    run(cargo["build", "--release", "--target", target])
    run(meson["setup", "build", "-Dtest_rust=false", "--reconfigure"])
    run(ninja["-C", "build"])

    rav1d = Path("target") / target / "release/dav1d"
    dav1d = Path("build") / "tools/dav1d"

    export_json_path = dir / f"benchmark-{"-".join(str(n) for n in threads)}.json"

    av1d_var = "av1d"
    threads_var = "threads"

    av1ds = [rav1d, dav1d]

    if not cache or not export_json_path.exists():
        run(hyperfine[
            "--warmup", "3",
            "--parameter-list", av1d_var, ",".join(str(path) for path in av1ds),
            "--parameter-list", threads_var, ",".join(str(threads) for threads in threads),
            f"{{{av1d_var}}} -q -i {str(video_path)} -o /dev/null --limit 1000 --threads {{{threads_var}}}",
            "--export-json", str(export_json_path)
        ])
    
    data = json.loads(export_json_path.read_text())
    results = data["results"]
    per_thread = {thread: {result["parameters"]["av1d"]: result for result in results if int(result["parameters"]["threads"]) == thread} for thread in threads}
    for thread, result in per_thread.items():
        rav1d_time = result[str(rav1d)]["mean"]
        dav1d_time = result[str(dav1d)]["mean"]
        diff = rav1d_time / dav1d_time
        percent = (diff - 1) * 100
        print(f"{thread:3} threads: {percent:4.1f}%, {rav1d_time:.3f} s, {dav1d_time:.3f} s")


if __name__ == "__main__":
    typer.run(main)
