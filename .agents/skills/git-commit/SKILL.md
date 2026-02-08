---
name: git-commit
description: commit changes
---

- Use English for commit messages.

## Conventional Commit Prefixes
Use the following prefixes for commit messages:

- `feat`: Use when adding new features, applying formatting-only visual changes, or improving performance.
- `fix`: Use for bug fixes.
- `docs`: Use when updating only documentation, such as README files or code comments.
- `refactor`: Use when improving the internal structure without changing external behavior.
- `test`: Use when adding or modifying tests.
- `chore`: Use for miscellaneous maintenance tasks that do not directly affect application behavior, including CI configuration and scripts.

## Breaking Changes
Indicate breaking changes by appending `!` to the prefix (e.g., `feat!`) or by including a `BREAKING CHANGE:` paragraph in the commit message body.
