# How to Re-Transpile `fn`s After Changes to the C code

1. Make your changes to the C code.
2. Run `./retranspile.sh transpile` to actually re-transpile.
3. For each `fn` `${fn_name}`,
    1. Copy `fn ${fn_name}` and any new callees to `retranspile/${fn_name}.fn.new`.
4. Run `./retranspile.sh stash`, `stash`ing all of the re-transpile changes.
5. For each `fn` `${fn_name}`,
    1. Run `./retranspile.sh fn-diff ${fn_name}`, saving `fn ${fn_name}`'s diff
       (it unfortunately shows a lot extra, too)
       from the `initial-transpile` to `retranspile/${fn_name}.fn.diff`.
    2. Replace the existing `fn` with the new version.
    3. Patch back the changes in `retranspile/${fn_name}.fn.diff`, probably manually.
    4. Run `./retranspile.sh commit ${fn_name}`, committing the re-transpiled changes.
6. Run `./retranspile.sh cleanup ${fn_name}` to cleanup intermediate files for `${fn_name}`,
   or just `./retranspile.sh cleanup` to cleanup them all.
