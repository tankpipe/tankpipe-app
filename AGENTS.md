<INSTRUCTIONS>
## Audience
- This file is for AI agents working in this repository.

## Required Workflow
- Before adding new code, search for an existing function or pattern to reuse.
- Add tests for new behavior or changes to existing behavior.
- Run the relevant test target(s) for the area you changed.
- Keep diffs minimal and scoped to the task.
- If switching a component to Svelte runes mode, update script/reactivity/props usage to runes-compatible patterns.
- Add frontend test code under the `tests` directory.

## Engineering Standards
- Always extract string literals for i18n.

## UI/Design Standards
- Prefer using existing approaches for new form components.
- Prefer consistency with existing style and design approaches when adding new UI items.
- Define all colors as CSS variables in `global.css`, with entries in both theme definitions.

</INSTRUCTIONS>
