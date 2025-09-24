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
import math
from pathlib import Path
import shutil
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
patchelf = local["patchelf"]

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
    error: None | ProcessExecutionError | RuntimeError
    rav1d: Path
    dav1d: Path

def build_commit(
    dir: Path,
    cache: bool,
    commit: str,
) -> Build:
    fix_arm_commit = "ad951f78"

    rust_toolchain_toml = """
[toolchain]
channel = "nightly-2025-05-01"
""".lstrip()

    resolved_commit = resolve_commit(commit)

    cached_error = dir / f"{resolved_commit}.error"
    cached_rav1d = dir / f"{resolved_commit}.rav1d"
    cached_dav1d = dir / f"{resolved_commit}.dav1d"
    cached_dav1d_so = dir / f"{resolved_commit}.dav1d.so"

    build = Build(
        commit=commit,
        resolved_commit=resolved_commit,
        error=None,
        rav1d=cached_rav1d,
        dav1d=cached_dav1d,
    )

    if cache and cached_error.exists():
        print(f"using cached {cached_error}")
        build.error = RuntimeError(cached_error.read_text())
        return build

    if cache and all(file.exists() for file in [cached_rav1d, cached_dav1d, cached_dav1d_so]):
        print(f"using cached {cached_rav1d}, {cached_dav1d}, {cached_dav1d_so}")
        return build
    
    target = host_target()
    head_commit = resolve_commit("HEAD")

    rav1d = Path("target") / target / "release/dav1d"
    dav1d = Path("build") / "tools/dav1d"
    dav1d_so = Path("build") / "src/libdav1d.so"

    stashed = run(git["stash", "push"]).strip() != "No local changes to save"
    if resolved_commit != head_commit:
        run(git["checkout", commit])
    run(git["cherry-pick", "--no-commit", "--strategy-option", "theirs", fix_arm_commit])
    Path("rust-toolchain.toml").write_text(rust_toolchain_toml)

    interrupt = None
    try:
        run(cargo["build", "--release", "--target", target])
        run(meson["setup", "build", "-Dtest_rust=false", "--reconfigure"])
        run(ninja["-C", "build"])
    except ProcessExecutionError as e:
        build.error = e
        cached_error.write_text(f"{e}")
        print(f"cached {cached_error}")
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
    
    if build.error is not None:
        return build

    rav1d.rename(cached_rav1d)
    print(f"cached {cached_rav1d}")
    dav1d.rename(cached_dav1d)
    print(f"cached {cached_dav1d}")

    # Need to handle different versions and symlinks.
    resolved_dav1d_so = dav1d_so.parent / dav1d_so.readlink()
    resolved_dav1d_so.resolve().rename(cached_dav1d_so)
    print(f"cached {cached_dav1d_so}")
    assert cached_dav1d_so.exists()

    # Need to update `dav1d` and `libdav1d.so`
    # to recognize the update name and location of `libdav1d.so`.
    run(patchelf["--set-rpath", "$ORIGIN", cached_dav1d])
    run(patchelf["--replace-needed", resolved_dav1d_so.name, cached_dav1d_so.name, cached_dav1d])
    run(patchelf["--set-soname", cached_dav1d_so.name, cached_dav1d_so])

    return build
    
@dataclass
class Benchmark:
    commit: str
    threads: int
    error: None | ProcessExecutionError | RuntimeError
    rav1d_time: float
    dav1d_time: float

    def diff(self) -> float:
        return (self.rav1d_time / self.dav1d_time) - 1

    def __str__(self) -> str:
        prefix = f"{self.commit}, {self.threads:3} threads: "
        if self.error is None:
            percent = self.diff() * 100
            return f"{prefix}{percent:4.1f}%, {self.rav1d_time:.3f} s, {self.dav1d_time:.3f} s"
        else:
            e = f"{self.error}"
            first_error = next(line for line in e.split("\n") if "error:" in line)
            return f"{prefix}error: {first_error}"

def benchmark_build(
    dir: Path,
    cache: bool,
    threads: list[int],
    video: Video,
    build: Build,
) -> Generator[Benchmark, None, None]:
    if build.error is not None:
        for thread in threads:
            yield Benchmark(
                commit=build.resolved_commit,
                threads=thread,
                error=build.error,
                rav1d_time=0,
                dav1d_time=0,
            )
        return

    av1d_var = "av1d"
    threads_var = "threads"

    export_json_path = dir / f"{build.resolved_commit}-{"-".join(str(n) for n in threads)}.benchmark.json"
    av1ds = [build.rav1d, build.dav1d]

    if cache and export_json_path.exists() and export_json_path.stat().st_size > 0:
        print(f"using cached {export_json_path}")
    else:
        run(hyperfine[
            "--show-output",
            "--warmup", "3",
            "--parameter-list", av1d_var, ",".join(str(path) for path in av1ds),
            "--parameter-list", threads_var, ",".join(str(threads) for threads in threads),
            f"{{{av1d_var}}} -q -i {str(video.path)} -o /dev/null --limit 1000 --threads {{{threads_var}}}",
            "--export-json", str(export_json_path)
        ])
        print(f"cached {export_json_path}")
    
    data = json.loads(export_json_path.read_text())
    results = data["results"]
    per_thread = {thread: {result["parameters"]["av1d"]: result for result in results if int(result["parameters"]["threads"]) == thread} for thread in threads}
    for thread, result in per_thread.items():
        yield Benchmark(
            commit=build.resolved_commit,
            threads=thread,
            error=None,
            rav1d_time=result[str(build.rav1d)]["mean"],
            dav1d_time=result[str(build.dav1d)]["mean"],
        )

def main(
    threads: Annotated[list[int], Option(help="list of number of threads to test with")],
    cache: Annotated[bool, Option(help="cache results")] = True,
    commit: Annotated[str, Option(help="git commit(s) to benchmark")] = "HEAD",
    diff_threshold: Annotated[float, Option(help="perf diff threshold to subdivide into narrower commits")] = 0.01
):
    threads.sort()

    dir = Path("benchmarks")
    video = download_video(dir)

    if ".." not in commit:
        build = build_commit(dir, cache, commit)
        for benchmark in benchmark_build(dir, cache, threads, video, build):
            print(benchmark)
    else:
        if len(threads) != 1:
            raise RuntimeError("can't bisect over multiple threads")
        thread = threads[0]

        output: str = git["rev-list", commit]()
        commits = [line.strip() for line in output.strip().split("\n")][::-1]
        
        benchmark_by_commit: dict[str, Benchmark] = {}

        def benchmark_one(index: int) -> Benchmark:
            commit = commits[index]
            if commit in benchmark_by_commit:
                return benchmark_by_commit[commit]
            build = build_commit(dir, cache, commit)
            benchmarks = list(benchmark_build(dir, cache, threads, video, build))
            assert(len(benchmarks) == 1)
            benchmark = benchmarks[0]
            benchmark_by_commit[commit] = benchmark
            print(f"{index}, {benchmark}")
            return benchmark

        def benchmark_range(first_index: int, last_index: int):
            count = last_index - first_index + 1
            if count <= 0:
                return
            elif count == 1:
                benchmark_one(first_index)
            elif count == 2:
                benchmark_one(first_index)
                benchmark_one(last_index)
            else:
                mid_index = (first_index + last_index) // 2
                first = benchmark_one(first_index)
                last = benchmark_one(last_index)
                if first.error is None and last.error is None:
                    diff_of_diff = abs(first.diff() - last.diff())
                    if diff_of_diff > diff_threshold:
                        benchmark_range(first_index, mid_index)
                        benchmark_range(mid_index, last_index)
                elif first.error is None:
                    benchmark_range(first_index, mid_index)
                elif last.error is None:
                    benchmark_range(mid_index, last_index)
        
        benchmark_range(0, len(commits) - 1)

        print()
        print(f"commit #, commit hash, threads: diff %, rav1d s, dav1d s")
        for i, commit in enumerate(commits):
            if commit in benchmark_by_commit:
                print(f"{i:4}, {benchmark_by_commit[commit]}")

if __name__ == "__main__":
    typer.run(main)
