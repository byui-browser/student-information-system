# Enrollment

## Purpose

Enrollment places students into sections. It enforces the rules for adding
and dropping classes, manages waitlists, and produces class rosters. It
depends on more domains than any other, so it is the best test of whether
the system fits together.

## Slices

| Slice | Owns |
| --- | --- |
| Registration | Add, drop, swap, withdraw, registration eligibility checks, student schedule |
| Waitlists and rosters | Waitlists, seat offers, class rosters, instructor roster views, enrollment counts |

## Core concepts

- **Enrollment:** a student's place in a section, with status (enrolled, waitlisted, dropped, withdrawn) and timestamps.
- **Registration window:** when a student may register, possibly by class standing or priority group.
- **Eligibility check:** the list of reasons a student may or may not add a section.
- **Waitlist entry:** a student's position in line for a full section.
- **Roster:** the list of students enrolled in a section.

## Features

### Foundation

- Add a section to a student's schedule.
- Drop a section.
- View a student's schedule for a term.
- View a section's roster.
- Enforce section capacity.
- Report seat counts to Catalog.

### Core

- Eligibility checks before adding, each with a clear reason when it fails:
  - registration window is open for this student
  - no registration hold (Finance, Advising, Records)
  - prerequisites met (Catalog + Records)
  - no time conflict with the student's other sections
  - within the term credit limit
  - not already enrolled in the same course
- Waitlist: join, leave, see position.
- When a seat opens, offer it to the next waitlisted student with a time limit to accept.
- Withdraw after the drop deadline (shows on transcript as `W`).
- Swap: drop one section and add another in one step, so the student never loses both.
- Shopping cart: plan sections, then register for all at once.
- Instructor and registrar overrides (capacity, prerequisite), recorded with a reason.

### Stretch

- Registration priority groups (e.g. seniors first).
- Schedule builder that suggests conflict-free combinations.
- Bulk enrollment by the registrar.
- Enrollment verification letter for a student.
- Notifications when a waitlist seat is offered.

## Business rules

- A student cannot exceed section capacity unless an authorized override exists.
- Drops before the add/drop deadline leave no record on the transcript; withdrawals after it do.
- No changes after the withdraw deadline without registrar approval.
- Waitlists are first come, first served unless an override says otherwise.
- Adding and dropping must be safe under concurrency: two students cannot both take the last seat.
- Every change to an enrollment is recorded with who made it and when.

## Interactions

| Needs from | What |
| --- | --- |
| Catalog | Sections, capacity, meeting times, prerequisites, term dates |
| Records | Student status, completed courses, and grades for prerequisite checks |
| Finance | Registration holds |
| Advising | Advising holds, program for priority rules |
| Identity | The current principal and their role |

| Provides to | What |
| --- | --- |
| Catalog | Seat counts |
| Records | Final enrollments at term end, withdrawals |
| Finance | Enrolled credits per student per term, for tuition |
| Advising | Current and planned courses for degree audit |

## Open questions

- How does Enrollment evaluate prerequisites: ask Catalog for the rule and Records for grades, or ask one service to decide?
- Do waitlist offers expire? How long?
- Is a "hold" checked once at add time, or also when the hold is placed after registration?
