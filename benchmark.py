#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "plumbum",
#     "typer",
# ]
# ///

from dataclasses import dataclass
import json
from pathlib import Path
from typing import Annotated, Generator
from urllib.parse import urlparse
from urllib.request import urlretrieve
import typer
from plumbum import ProcessExecutionError, local
from plumbum.commands.base import BoundCommand
from plumbum.machines.local import LocalCommand
from typer import Option

def run(cmd: LocalCommand | BoundCommand) -> str:
    print(cmd)
    return cmd()

# commands used
git = local["git"]
rustc = local["rustc"]
cargo = local["cargo"]
meson = local["meson"]
ninja = local["ninja"]
hyperfine = local["hyperfine"]

def host_target() -> str:
    output: str = rustc["-vV"]()
    prefix = "host: "
    for line in output.split("\n"):
        if line.startswith(prefix):
            return line[len(prefix):]
    raise RuntimeError("rustc host target not found")

def resolve_commit(commit: str) -> str:
    output: str = git["rev-parse", "--short", commit]()
    return output.strip()

@dataclass
class Video:
    url: str
    path: Path

def download_video(dir: Path) -> Video:
    url = "http://download.opencontent.netflix.com.s3.amazonaws.com/AV1/Chimera/Old/Chimera-AV1-8bit-1280x720-3363kbps.ivf"
    path = Path(urlparse(url).path)
    path = dir / path.name

    dir.mkdir(exist_ok=True)
    if not path.exists():
        urlretrieve(url, path)
    
    return Video(url=url, path=path)

@dataclass
class Build:
    commit: str
    resolved_commit: str
    error: None | ProcessExecutionError
    rav1d: Path
    dav1d: Path

def build_commit(
    commit: str,
) -> Build:
    fix_arm_commit = "9ecc4e4b"

    rust_toolchain_toml = """
[toolchain]
channel = "nightly-2025-05-01"
""".lstrip()
    
    target = host_target()
    head_commit = resolve_commit("HEAD")

    resolved_commit = resolve_commit(commit)

    stashed = run(git["stash", "push"]).strip() != "No local changes to save"
    if resolved_commit != head_commit:
        run(git["checkout", commit])
    run(git["cherry-pick", "--no-commit", fix_arm_commit])
    Path("rust-toolchain.toml").write_text(rust_toolchain_toml)

    error = None
    interrupt = None
    try:
        run(cargo["build", "--release", "--target", target])
        run(meson["setup", "build", "-Dtest_rust=false", "--reconfigure"])
        run(ninja["-C", "build"])
    except ProcessExecutionError as e:
        error = e
        print(f"skipping {commit} due to build error: {e}")
    except KeyboardInterrupt as e:
        interrupt = e

    run(git["stash", "push"])
    run(git["stash", "drop"])
    if resolved_commit != head_commit:
        run(git["checkout", "-"])
    if stashed:
        run(git["stash", "pop"])
    
    # Allow us to reset git state if interrupted.
    if interrupt is not None:
        raise interrupt

    return Build(
        commit=commit,
        resolved_commit=resolved_commit,
        error=error,
        rav1d=Path("target") / target / "release/dav1d",
        dav1d=Path("build") / "tools/dav1d",
    )
    
@dataclass
class Benchmark:
    commit: str
    threads: int
    rav1d_time: float
    dav1d_time: float

    def diff(self) -> float:
        return (self.rav1d_time / self.dav1d_time) - 1

    def __str__(self) -> str:
        percent = self.diff() * 100
        return f"{self.commit}, {self.threads:3} threads: {percent:4.1f}%, {self.rav1d_time:.3f} s, {self.dav1d_time:.3f} s"

def benchmark_build(
    dir: Path,
    cache: bool,
    threads: list[int],
    video: Video,
    build: Build,
) -> Generator[Benchmark, None, None]:
    if build.error is not None:
        return

    av1d_var = "av1d"
    threads_var = "threads"

    export_json_path = dir / f"benchmark-{build.resolved_commit}-{"-".join(str(n) for n in threads)}.json"
    av1ds = [build.rav1d, build.dav1d]

    if cache and export_json_path.exists():
        print(f"cached {export_json_path}")
    else:
        run(hyperfine[
            "--warmup", "3",
            "--parameter-list", av1d_var, ",".join(str(path) for path in av1ds),
            "--parameter-list", threads_var, ",".join(str(threads) for threads in threads),
            f"{{{av1d_var}}} -q -i {str(video.path)} -o /dev/null --limit 1000 --threads {{{threads_var}}}",
            "--export-json", str(export_json_path)
        ])
    
    data = json.loads(export_json_path.read_text())
    results = data["results"]
    per_thread = {thread: {result["parameters"]["av1d"]: result for result in results if int(result["parameters"]["threads"]) == thread} for thread in threads}
    for thread, result in per_thread.items():
        yield Benchmark(
            commit=build.resolved_commit,
            threads=thread,
            rav1d_time=result[str(build.rav1d)]["mean"],
            dav1d_time=result[str(build.dav1d)]["mean"],
        )

def main(
    threads: Annotated[list[int], Option(help="list of number of threads to test with")],
    cache: Annotated[bool, Option(help="cache results")] = True,
    commit: Annotated[str, Option(help="git commit(s) to benchmark")] = "HEAD",
    diff_threshold: Annotated[float, Option(help="perf diff threshold to subdivide into narrower commits")] = 0.01
):
    dir = Path("benchmarks")
    video = download_video(dir)

    if ".." not in commit:
        build = build_commit(commit)
        for benchmark in benchmark_build(dir, cache, threads, video, build):
            print(benchmark)
    else:
        if len(threads) != 1:
            raise RuntimeError("can't bisect over multiple threads")
        thread = threads[0]

        output: str = git["rev-list", commit]()
        commits = [line.strip() for line in output.strip().split("\n")]
        
        benchmark_by_commit: dict[str, None | Benchmark] = {}

        def benchmark_one(index: int) -> None | Benchmark:
            commit = commits[index]
            if commit in benchmark_by_commit:
                return benchmark_by_commit[commit]
            build = build_commit(commit)
            benchmarks = list(benchmark_build(dir, cache, threads, video, build))
            if len(benchmarks) == 0:
                # if there was an error
                benchmark = None
            elif len(benchmarks) == 1:
                benchmark = benchmarks[0]
            else:
                assert(False)
            benchmark_by_commit[commit] = benchmark
            print(f"{index}, {benchmark}")
            return benchmark

        def benchmark_range(first_index: int, last_index: int):
            count = last_index - first_index + 1
            if count <= 0:
                return
            elif count == 1:
                benchmark_one(first_index)
            else:
                mid_index = (first_index + last_index) // 2
                first = benchmark_one(first_index)
                last = benchmark_one(last_index)
                if first is not None and last is not None:
                    diff_of_diff = abs(first.diff() - last.diff())
                    if diff_of_diff > diff_threshold:
                        benchmark_range(mid_index, last_index)
                        benchmark_range(first_index, mid_index)
                elif first is not None:
                    benchmark_range(first_index, mid_index)
                elif last is not None:
                    benchmark_range(mid_index, last_index)
        
        benchmark_range(0, len(commits) - 1)

        for i, commit in enumerate(commits):
            print(f"{i}, {benchmark_by_commit[commit]}")

if __name__ == "__main__":
    typer.run(main)
