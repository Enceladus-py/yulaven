---
description: Conventional commits workflow and auto-committer
---

# Git Workflow: Conventional Commits

This project follows the **Conventional Commits** standard to ensure a clean, readable, and machine-parsable git history. This is critical for generating changelogs and understanding the sequence of minimal steps during development.

## The Standard

Commit messages must be formatted as follows:
`<type>[optional scope]: <description>`

### Allowed Types:
*   `feat:` A new feature or gameplay mechanic (e.g., `feat: add magic wand weapon`)
*   `fix:` A bug fix (e.g., `fix: resolve enemy spawn panic`)
*   `refactor:` A code change that neither fixes a bug nor adds a feature (e.g., `refactor: migrate collision to spatial grid`)
*   `chore:` Updates to build tools, dependencies, or repository configuration (e.g., `chore: update bevy to 0.18`)
*   `docs:` Documentation only changes (e.g., `docs: update survival game design manifesto`)
*   `style:` Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)
*   `test:` Adding missing tests or correcting existing tests

## Turbo Workflow

Use the following step to auto-stage and prompt for a commit message when you are ready to wrap up a minimal step.

// turbo-all
1. Stage all changes
```bash
git add .
```

2. Request a commit message. (The agent should run `git status` or `git diff --cached` to see changes, generate an appropriate Conventional Commit message, and run the commit command).
```bash
# Agent will infer the message and run:
# git commit -m "type(scope): description"
echo "Ready to commit. Please generate and run the git commit command."
```
