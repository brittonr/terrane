# dead-code-elimination dedup delta

## MODIFIED Requirements

### Requirement: Live Definitions Never Dead
r[dce.live-def-never-dead]

An instruction whose defined variable is live-out MUST never be classified
as dead, regardless of opcode.
