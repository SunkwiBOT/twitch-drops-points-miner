## What and why

<!-- What this pull request changes, and why. Link the issue it closes: "Closes #123". -->

## Tested on

<!-- Desktop app (which system) and/or Docker (over plain HTTP or behind an HTTPS reverse proxy). For an interface change, add a screenshot. -->

## Checklist

- [ ] `cargo fmt --all --check` and `cargo clippy --workspace --all-targets --locked -- -D warnings` pass.
- [ ] `npm run verify` passes.
- [ ] After a change to a type the interface sees, I ran `npm run bindings`.
- [ ] Every new or changed text is in all 29 files of `messages/`, with the "mine / mining" wording.
- [ ] Comments are rare: only a fact the code cannot show, starting with `!`.
- [ ] `identifier` and the updater `pubkey` in `src-tauri/tauri.conf.json` are unchanged: installed apps would stop updating.
