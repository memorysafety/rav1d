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
    
    for commit in [
        "ad951f78", # fix arm
        "3114c84b", # fix `goto error` error
        "d42c04ee", # fix `ALLOC_BLOCK` error
    ]:
        run(git["cherry-pick", "--no-commit", "--strategy-option", "theirs", commit])

    # Use a consistent toolchain.
    Path("rust-toolchain.toml").write_text(rust_toolchain_toml)

    # `proc-macro2` has compile errors on older versions.
    run(cargo["update", "--package", "proc-macro2"])
    
    # Add `#![feature(let_chains)]` to `c2rust-lib.rs`.
    lib_rs = Path("c2rust-lib.rs")
    if lib_rs.exists():
        rs = lib_rs.read_text()
        rs = f"#![feature(let_chains)]\n{rs}"
        lib_rs.write_text(rs)
    
    # Delete `rav1d`/Rust stuff from meson.
    tests_meson_build = Path("tests/meson.build")
    if tests_meson_build.exists():
        meson_build = tests_meson_build.read_text()
        lines = meson_build.split("\n")
        
        filtered_lines = []
        skipping = False

        for line in lines:
            if skipping:
                if line.startswith("endif"):
                    skipping = False # stop skipping after `endif`
                continue
            if any(s in line for s in [
                "if get_option('test_rust_path')",
                "if get_option('test_rust')",
                "= get_option('test_rust_path')",
                "= get_option('test_rust')",
            ]):
                skipping = True # start skipping
                continue
            filtered_lines.append(line)

        lines = filtered_lines

        meson_build = "\n".join(lines)
        tests_meson_build.write_text(meson_build)

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
            lines = e.split("\n")
            first_error = None
            if first_error is None:
                first_error = next((line for line in lines if "error:" in line), None)
            if first_error is None:
                first_error = next((line for line in lines if "panicked at" in line), None)
            if first_error is None:
                first_error = lines[0]
            first_error = first_error.strip()
            return f"{prefix}{first_error}"

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
        try:
            run(hyperfine[
                "--show-output",
                "--warmup", "3",
                "--parameter-list", av1d_var, ",".join(str(path) for path in av1ds),
                "--parameter-list", threads_var, ",".join(str(threads) for threads in threads),
                f"{{{av1d_var}}} -q -i {str(video.path)} -o /dev/null --limit 1000 --threads {{{threads_var}}}",
                "--export-json", str(export_json_path)
            ])
            print(f"cached {export_json_path}")
        except ProcessExecutionError as e:
            cached_error = dir / f"{build.resolved_commit}.error"
            cached_error.write_text(f"{e}")
            print(f"cached {cached_error}")
            print(f"skipping {build.commit} due to runtime error: {e}")
            for thread in threads:
                yield Benchmark(
                    commit=build.resolved_commit,
                    threads=thread,
                    error=e,
                    rav1d_time=0,
                    dav1d_time=0,
                )
            return
        
    
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
    merges: Annotated[bool, Option(help="only look at merge commits")] = False,
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

        output: str = run(git["rev-list", *(["--merges"] if merges else []), commit])
        commits = [line.strip() for line in output.strip().split("\n")][::-1]
        print(f"benchmarking {commit}: {len(commits)} commits")
        
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
                recurse = True
                if first.error is None and last.error is None:
                    diff_of_diff = abs(first.diff() - last.diff())
                    percent = lambda x: f"{x * 100:.1f}%"
                    recurse = diff_of_diff > diff_threshold
                    if recurse:
                        print(f"{percent(diff_of_diff)} > {percent(diff_threshold)}, so recursing")
                    else:
                        print(f"{percent(diff_of_diff)} <= {percent(diff_threshold)}, so not recursing")
                if first.error is not None and last.error is not None:
                    def find_assert(error: ProcessExecutionError | RuntimeError) -> str:
                        s = f"{error}"
                        lines = s.split("\n")
                        lines = [line.strip() for line in lines if "assertion failed:" in line]
                        lines = sorted(set(lines))
                        return "\n".join(lines)
                    first_assert = find_assert(first.error)
                    last_assert = find_assert(last.error)
                    if first_assert != "" and last_assert != "" and first_assert == last_assert:
                        print(f"same error, so not recursing: {first_assert}")
                        recurse = False
                if recurse:
                    benchmark_range(first_index, mid_index)
                    benchmark_range(mid_index, last_index)
        
        benchmark_range(0, len(commits) - 1)

        print()
        print(f"commit #, commit hash, threads: diff %, rav1d s, dav1d s")
        for i, commit in enumerate(commits):
            if commit in benchmark_by_commit:
                print(f"{i:4}, {benchmark_by_commit[commit]}")

if __name__ == "__main__":
    typer.run(main)
