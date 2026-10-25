# Reviewing a kata fix

A kata fix PR changes one kata's `src/` (plus a `DESIGN.md` when that kata's README asks for one, and nothing else under `katas/`) and answers that kata's README questions in the PR description. Review it as a Rust API code review: idioms, ownership, the [API Guidelines](https://rust-lang.github.io/api-guidelines/).

Each item passes or fails. Report every item, with a one-line reason for each fail.

## 1. Tests unchanged, and passing
- [ ] No file under `katas/*/*/tests/` changed, and neither did `Cargo.toml`.
      Check: `git diff --name-only origin/main...HEAD -- 'katas/*/*/tests/*' 'katas/*/*/Cargo.toml'` prints nothing.
      Exception: when a kata's README **Task** says the fix is in Cargo.toml, that kata's Cargo.toml, Cargo.lock and .gitignore may change too.
- [ ] Only one kata changed. Check: `git diff --name-only origin/main...HEAD -- katas/ | cut -d/ -f1-3 | sort -u` prints one directory.
- [ ] `cargo test` passes in that kata (CI's "katas" job is green).

## 2. The fix respects the kata's rule
- [ ] It obeys the kata README's **Task** line (e.g. "Don't change the test").
- [ ] The fix is one of the kata's named fixes, or an equivalent the PR justifies.
- [ ] No needless `.clone()`: allowed when the README names it or the signature needs an owned value; a fail when a borrow would work, or when it copies more than needed (`to_vec()`, cloning a collection to get one item).
- [ ] No dodging lifetimes: no `Box::leak`, no swapping in a literal to make the value `'static`, no new `'static` in a signature, no `unsafe`, `transmute` or `Rc<RefCell<…>>` added only to quiet the borrow checker.
- [ ] No test-gaming: the code doesn't hard-code the value the test expects.

## 3. Clean
- [ ] `cargo fmt --manifest-path <kata>/Cargo.toml --check`
- [ ] `cargo clippy --manifest-path <kata>/Cargo.toml --all-targets -- -D warnings`
- [ ] No `#[allow(...)]` added to silence a lint.

## 4. Answers in the author's own words
- [ ] The PR description answers every question under the README's **Answer in the PR**, numbered the same way.
- [ ] Answers use the kata's terms (variable, value, place, pointer, owner, borrow, lifetime), not just restate the question or paste compiler output.
- [ ] Where the README asks for the other fix's cost, the answer names a concrete cost (an allocation, who owns what, how long a borrow lives).
- [ ] Answers are the author's own words: the reviewer doesn't rewrite them, but asks a question when an answer is wrong or vague.

## 5. New kata PRs only (skip for fix PRs)
- [ ] The kata's README has a `## Related concepts` section, or the line `None: no close siblings.`
- [ ] Each sibling has a one-sentence definition, its main use case and a snippet of at most 5 lines, and there are at most 4 siblings.
- [ ] No snippet gives away the kata's fix.

## Verdict
Approve only when every item that applies passes. Otherwise request changes, listing the failed items by number (e.g. "2.4: `Box::leak` makes the string `'static`; which variable should own it instead?"). Hint at the rule; don't give the fix.
