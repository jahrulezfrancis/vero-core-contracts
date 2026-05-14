# Vero Wave Program — Contribution Plan

## What is the Wave Program?

The Wave Program is Vero's structured contribution cycle. Maintainers open scoped GitHub issues at the start of each sprint ("wave"). Contributors pick up those issues, submit PRs, and earn on-chain rewards once a quorum of Guardians verifies the work via the VeroOracle contract.

Each wave runs for **2 weeks**. At the end of a wave, unresolved tasks roll over or are re-scoped for the next cycle.

---

## How a Wave Works

```
Maintainer opens issue  ──►  Relayer calls register_task(github_id)
                                        │
Contributor submits PR  ──►  Guardians review & call vote(guardian, github_id)
                                        │
                             3 votes reached  ──►  "verified" event emitted
                                        │
                             Reward Distributor releases payment
```

---

## Types of Work

### 1. Bug Fixes
Scoped, well-defined issues with a clear acceptance criterion.
- Label: `bug`
- Reward tier: **Small** (1–5 XLM equivalent)
- Example: "Fix off-by-one in vote threshold check"

### 2. New Features
Larger issues that extend contract functionality.
- Label: `feature`
- Reward tier: **Medium–Large** (10–50 XLM equivalent)
- Example: "Add configurable vote threshold per task"
- Requires: design comment approved by Admin before work starts

### 3. Documentation
Improving READMEs, inline comments, architecture diagrams, or runbooks.
- Label: `docs`
- Reward tier: **Small** (1–3 XLM equivalent)
- Example: "Document Guardian onboarding flow"

### 4. Testing
Adding unit tests, integration tests, or fuzz targets for existing logic.
- Label: `testing`
- Reward tier: **Small–Medium** (3–10 XLM equivalent)
- Example: "Add integration test for double-vote guard"

### 5. Security & Auditing
Reviewing contract logic for vulnerabilities, writing audit reports.
- Label: `security`
- Reward tier: **Large** (25–100 XLM equivalent)
- Requires: Guardian-level trust; findings reviewed by Admin

---

## Contribution Rules

1. **One contributor per task** — first to comment "claiming" on the issue gets it.
2. **PR must reference the issue** — `Closes #<issue_number>` in the PR body.
3. **Guardian review is final** — once 3 Guardians vote `verified`, the task is locked.
4. **No self-voting** — Guardians cannot vote on tasks they authored.
5. **Wave deadline** — PRs not merged by wave end are deferred; partial work is not rewarded.

---

## Becoming a Guardian

Guardians are trusted developers whitelisted by the Admin on-chain:

```bash
soroban contract invoke --id <CONTRACT_ID> \
  --fn add_guardian -- --guardian <YOUR_ADDRESS>
```

To apply, open an issue with the label `guardian-application` and include:
- Your GitHub handle
- 3 examples of prior open-source contributions
- Your Stellar address

Applications are reviewed by existing Guardians each wave.

---

## Roadmap

| Wave | Focus |
|------|-------|
| 1 | Core contract: init, guardian, task, vote |
| 2 | Configurable thresholds + multi-contract support |
| 3 | Reward Distributor contract |
| 4 | Relayer service (off-chain, TypeScript) |
| 5 | Frontend dashboard for task tracking |
