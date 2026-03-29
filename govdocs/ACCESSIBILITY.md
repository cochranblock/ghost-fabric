<!-- Unlicense — cochranblock.org -->

# Accessibility — Section 508 / WCAG Compliance

## Product Type

Ghost-fabric is a **CLI tool**. No web interface. No GUI. Section 508 applies to the command-line experience.

## CLI Accessibility Assessment

### Help Text Completeness

| Feature | Status | Evidence |
|---------|--------|---------|
| `--help` flag | Present | clap auto-generates help for all subcommands |
| `-h` short flag | Present | clap provides both forms |
| `--version` / `-V` | Present | Prints `ghost-fabric 0.1.0` |
| Subcommand help | Present | `ghost-fabric help start`, `ghost-fabric help status`, `ghost-fabric help init` |
| Unknown command error | Clear | clap prints "error: unrecognized subcommand" with suggestions |

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Error (no config found, init failure) |
| 2 | CLI usage error (clap) |

Exit codes follow Unix conventions. Screen readers and automation tools can interpret them.

### Error Message Clarity

| Scenario | Message | Assessment |
|----------|---------|------------|
| No config, run status | "No node config. Run `ghost-fabric init` first." | Clear, actionable |
| No config, run start | "No node config found. Run `ghost-fabric init` first." | Clear, actionable |
| Invalid subcommand | clap error with valid options listed | Clear |
| No arguments | Version + "Run `ghost-fabric --help` for usage." | Clear |

### Output Format

- All output is plain text to stdout/stderr
- No color codes that would be invisible to screen readers (clap respects `NO_COLOR`)
- No progress bars or cursor manipulation
- No interactive prompts — all commands are non-interactive

### Screen Reader Compatibility

- All output is line-oriented text — fully compatible with screen readers
- No ANSI escape sequences in ghost-fabric output
- clap uses ANSI colors for help text but respects `NO_COLOR` environment variable

## Web / GUI Components

None. If a web interface is added in the future, it must meet WCAG 2.1 AA.
