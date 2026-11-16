# Finance

## Purpose

Finance tracks what each student owes and has paid, places holds that block
actions such as registration, and manages financial aid awards. Money flows
only through this domain.

## Slices

| Slice | Owns |
| --- | --- |
| Billing and payments | Student accounts, charges, payments, refunds, balance, statements, tuition calculation |
| Holds and financial aid | Holds (from any office), hold types and effects, aid awards, disbursement to accounts |

## Core concepts

- **Student account:** the ledger for one student.
- **Charge:** an amount owed, such as tuition, a course fee, or a late fee.
- **Payment:** an amount received, with method and reference.
- **Refund:** money returned when a credit balance exists.
- **Balance:** charges minus payments and aid. Negative means a credit.
- **Hold:** a block on a student's account with a type, reason, the office that placed it, and what it blocks (registration, transcripts, diploma).
- **Aid award:** grant, scholarship, or loan for a term, which reduces the balance when disbursed.

## Features

### Foundation

- View a student's account: charges, payments, and balance.
- Post a charge.
- Record a payment.
- Place and release a hold.
- Answer "does this student have a registration hold?" for Enrollment.

### Core

- Calculate tuition from enrolled credits (from Enrollment), with a flat rate above a credit threshold.
- Course fees from Catalog.
- Adjust tuition automatically when a student drops before the refund deadline.
- Automatically place a financial hold when the balance is past due.
- Holds placed by other offices (Advising, Records, Admissions) with their own types.
- Term statements a student can view or download.
- Payment plans: split a term's balance into installments.
- Award and disburse financial aid.

### Stretch

- Online payment through a payment provider in test mode.
- Refund calculation by withdrawal date (100%, 50%, 0%).
- Third-party sponsors (an employer pays part of the bill).
- 1098-T style annual tax summary.
- Aid eligibility rules (minimum enrolled credits, satisfactory academic progress).

## Business rules

- Store money as integer cents, never floating point.
- Ledger entries are never edited or deleted; corrections are new entries.
- Only the office that placed a hold (or an admin) can release it.
- A student with a registration hold cannot add sections.
- Aid is disbursed only if the student is enrolled in the required credits.
- Every ledger entry and hold change is audited.

## Interactions

| Needs from | What |
| --- | --- |
| Enrollment | Enrolled credits per student per term, drops and withdrawals |
| Catalog | Course fees, term dates (refund deadlines) |
| Records | Student status and name |
| Identity | Staff roles (finance, registrar, advisor) |

| Provides to | What |
| --- | --- |
| Enrollment | Registration holds |
| Records | Transcript holds |
| Advising | Holds placed by advisors |
| Students (clients) | Balance, statements, payment history |

## Open questions

- Is Finance the only owner of holds, or does each office keep its own and Finance only owns financial holds?
- Is tuition recalculated on every enrollment change, or once at a census date?
- What are the tuition rates and fee structure in the seed data?
