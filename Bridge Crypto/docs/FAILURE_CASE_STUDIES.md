# Failure Case Studies

### Valid Actions, Incorrect Execution, and the Absence of Non-Execution

This document examines several major crypto failures through a single lens:

> **The system treated execution as correct because invocation was valid — and had no way to represent non-execution as a correct outcome.**

Each case is structured to isolate that failure mode.

---

## 1. The DAO Hack (Ethereum, 2016)

### What happened

The DAO was a smart contract–based investment vehicle on Ethereum.
An attacker exploited a reentrancy vulnerability in the withdrawal logic, repeatedly draining funds into a child DAO.

Approximately **$60M worth of ETH** was extracted.

### Why it was considered "valid"

* The attacker invoked the withdrawal function using valid transactions.
* Each call complied with the contract's rules as written.
* The Ethereum protocol correctly executed each transaction.
* No consensus rules were violated.

From the system's perspective, execution was correct.

### Why non-execution was the correct outcome

The withdrawals, while valid under the contract logic, violated the intended economic and governance meaning of the system:

* The funds were not meant to be withdrawn repeatedly before balances updated.
* The execution sequence exploited momentum, not authorization.
* Continuing execution amplified damage with each valid call.

The correct system-level outcome was **not to continue executing valid withdrawals under these conditions**.

### How IIC reframes the failure

IIC does not claim it would have "prevented" the exploit.

Instead, it identifies the missing representational capacity:

* The system had no way to treat "do not continue execution" as a correct state.
* Valid invocation automatically implied obligation to execute.
* Non-execution was not expressible as correctness — only as failure or intervention.

IIC names that gap.

---

## 2. Beanstalk Governance Attack (2022)

### What happened

An attacker used a flash loan to temporarily acquire a majority of governance voting power in the Beanstalk protocol.

Within a single transaction, they:

1. Borrowed funds
2. Passed a malicious governance proposal
3. Executed the proposal
4. Repaid the loan

Approximately **$182M** was drained.

### Why it was considered "valid"

* Voting power was legitimately acquired under protocol rules.
* The proposal passed according to governance thresholds.
* Execution followed immediately, as designed.
* All actions occurred atomically and validly.

The governance system worked exactly as specified.

### Why non-execution was the correct outcome

While governance inputs were valid, execution relied on an implicit assumption:

> A passed proposal should execute immediately.

In this context:

* Voting power was transient and adversarial.
* There was no opportunity for review or reconsideration.
* Immediate execution converted temporary influence into irreversible action.

The correct outcome was **not to execute immediately**, despite procedural validity.

### How IIC reframes the failure

IIC highlights that the system lacked a way to distinguish between:

* *Valid governance outcome* and
* *Correct moment to execute*

The failure was not bad governance logic, but **execution-by-default**.

IIC provides a conceptual boundary where non-execution is allowed to be correct once risk is detected.

---

## 3. Wormhole Bridge Exploit (2022)

### What happened

An attacker exploited a verification flaw in the Wormhole cross-chain bridge, forging a message that appeared to authorize minting wrapped ETH on Solana.

Approximately **$320M** was minted without backing.

### Why it was considered "valid"

* The forged message passed verification checks.
* The bridge logic treated the message as authentic.
* Minting followed automatically upon verification.
* No external approvals were required post-validation.

Execution followed from a valid-looking message.

### Why non-execution was the correct outcome

Even if a message appears valid:

* Cross-chain minting is irreversible and high-risk.
* Execution commits value immediately.
* There was no secondary moment for judgment or restraint.

The correct outcome was **not to mint**, even though validation logic returned success.

### How IIC reframes the failure

IIC does not replace verification.

It reframes the assumption that:

> Successful validation obligates execution.

Under IIC, validation success does not force continuation.
Non-execution remains a valid, correct outcome when the cost of error is catastrophic.

---

## 4. Cascading Liquidations (Multiple Protocols)

### What happened

During periods of rapid market movement, price oracle updates triggered mass liquidations across DeFi lending platforms.

Each liquidation was valid individually, but collectively they:

* Amplified price crashes
* Created feedback loops
* Caused systemic stress and insolvency

Notable incidents include the March 2020 "Black Thursday" on MakerDAO, where over $8M in ETH was liquidated for $0 due to network congestion.

### Why it was considered "valid"

* Oracle prices crossed liquidation thresholds.
* Smart contracts executed liquidations as programmed.
* Each liquidation followed protocol rules.
* Automation ensured immediate response.

No individual action was incorrect.

### Why non-execution was the correct outcome

At the system level:

* Immediate liquidation during extreme volatility worsened outcomes.
* Execution momentum compounded harm.
* Slowing or halting execution temporarily would have preserved system integrity.

The correct response was **not continuous execution**, even though triggers were valid.

### How IIC reframes the failure

IIC identifies the absence of a system state where:

* "We should not act right now" is correct
* Not executing is not treated as malfunction
* Integrity outweighs immediacy

The issue was not liquidation logic, but the assumption that valid triggers must execute instantly.

---

## Summary Observation

Across all cases:

* Actions were valid
* Execution was correct by protocol rules
* Outcomes were catastrophic

The shared failure mode was not logic error, but **assumed continuation**.

IIC does not replace governance, validation, or automation.

It provides something these systems lacked:

> A way to represent **non-execution as correctness** in irreversible, high-risk contexts.

---

## Related Documents

* [DAO Governance Gate](dao-governance-gate.md) — Illustrative flow for IIC-aware governance
* [Treasury Attestation](treasury-attestation.md) — Semantic approval for high-value execution

---

*Bridge — governing execution before it becomes irreversible.*
