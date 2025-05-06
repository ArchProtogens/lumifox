# Commit Message Guidelines for AI

Below is a concise guide you can feed an AI to generate commit messages that are clear, concise, and informative. It distills principles from Chris Beams, the Conventional Commits spec, Atlassian, and other industry best practices into a single prompt.

## Key Principles

- **Subject line brevity**: Keep the subject under 50 characters, capitalize it, do not end with a period, and use the imperative mood. citeturn0search0
- **Separate subject and body**: Always put a blank line between the one‑line summary and the detailed explanation. citeturn0search0
- **Explain “what” and “why”**: In the body, describe what changed and, more importantly, why—not how (the diff shows implementation). citeturn0search7

## Structure

Follow the Conventional Commits format whenever possible:

```text
<type>[optional scope]: <short description>

[optional detailed explanation]

[optional footer(s)]
```

- Types include `feat`, `fix`, `docs`, `chore`, etc. citeturn0search2
- Scope is the affected module or component in parentheses. citeturn0search8
- Footers can reference issues or breaking changes. citeturn0search5

## Style Tips

- **Wrapping**: Limit body lines to about 72 characters. citeturn0search0
- **Spelling & Punctuation**: Avoid unnecessary capitalization and trailing punctuation in the subject. citeturn0search4
- **Tone**: Use an active, imperative tone (“Add feature,” “Fix bug”) to make the history actionable. citeturn0search7

## Clarity & Brevity

- **Omit needless words**: Be succinct—assume reviewers can read the code; they need context, not prose. citeturn0search15
- **Consistency**: Stick to a team‑agreed style, so history remains uniform. citeturn0search1

---

```text
You are an AI assistant tasked with drafting Git commit messages. When provided with a code diff or a change description, follow these guidelines:

1. SUBJECT (limit to 50 characters)
   • Use the imperative mood (e.g., “Add”, “Fix”, “Update”).
   • Capitalize the first letter; do not end with a period.
   • Summarize what changed, not how.

2. BLANK LINE
   • Separate the subject from the body with a single blank line.

3. BODY (wrap at ~72 characters)
   • Explain WHAT changed and, crucially, WHY it was done.
   • Do not describe how—the diff covers implementation.
   • If relevant, reference issue numbers (e.g., “Closes #123”).

4. FOOTER (optional)
   • For breaking changes, start with “BREAKING CHANGE: ” followed by details.
   • For issue tracking, use “Fixes #<issue>” or similar.

5. CONVENTIONAL COMMITS (optional but recommended)
   • Prefix with a type and optional scope:
     `<type>(<scope>): <description>`
     Types: feat, fix, docs, style, refactor, perf, test, chore, etc.

Always produce a well‑formatted commit message that adheres to these rules, keeping it concise, consistent, and informative.
```
