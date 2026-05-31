# .agents

All detailed agent guidance for `hinode-efi` lives here.

Root-level `AGENTS.md`, `CLAUDE.md`, and `SKILL.md` are intentionally tiny import
shims.

## Main file

```text
.agents/AGENTS.md
```

## Skill index

```text
.agents/SKILL.md
```

This file imports the split skill files under `.agents/*/SKILL.md`.

## Focused skills

```text
.agents/hinode-efi/SKILL.md
.agents/rust-uefi/SKILL.md
.agents/qemu-ci/SKILL.md
.agents/agent-collaboration/SKILL.md
```

## Rule

`.agents/AGENTS.md` is canonical.

Focused `SKILL.md` files may add detail, but must not contradict
`.agents/AGENTS.md`.
