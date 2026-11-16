# Admissions

## Purpose

Admissions manages the path from applicant to student: collecting
applications, reviewing them, making decisions, and handing admitted
students off to Records, Identity, and Advising. Applicants are not yet
students and have their own limited access.

## Slices

| Slice | Owns |
| --- | --- |
| Applications | Applicant accounts, application forms, document uploads, application status for the applicant, application deadlines |
| Review and decisions | Review queues, reviewer scoring, decisions, decision letters, deposit and intent to enroll, handoff to Records |

## Core concepts

- **Applicant:** a person applying, who may not yet have a student record.
- **Application:** one application to one program for one entry term, with status (draft, submitted, under review, decided).
- **Application requirement:** something an application needs, such as a transcript, test score, essay, or recommendation.
- **Review:** a reviewer's score and comments on an application.
- **Decision:** admit, deny, or waitlist, with a decision date.
- **Intent to enroll:** an admitted applicant accepting the offer (optionally with a deposit).

## Features

### Foundation

- Applicant creates and submits an application for a program and term.
- Staff list and view applications.
- Record a decision: admit, deny, or waitlist.
- Applicant sees their application status and decision.

### Core

- Save a draft application and return later.
- Checklist of required items per program, with completion status.
- Upload supporting documents.
- Review queue: assign applications to reviewers; reviewers score and comment.
- Status workflow: draft → submitted → under review → decided, with valid transitions only.
- Admitted applicant accepts or declines the offer.
- On acceptance, create the student record (Records), the user account (Identity), and the declared program (Advising).
- Application deadlines per term.

### Stretch

- Application fee and deposit, posted to Finance.
- Decision letters generated from templates.
- Reporting: applications, admits, and yield by program and term.
- Transfer applicant credit evaluation (feeds Records' transfer credit).
- Bulk decision release on a set date.

## Business rules

- An applicant may have only one active application per program per term.
- Applications cannot be edited after submission except by staff.
- Decisions require a completed review.
- Final decisions (admit, deny) cannot be reversed without an admin and a recorded reason.
- Applicant data is visible only to the applicant and admissions staff.
- An admitted applicant becomes a student exactly once, even if the handoff is retried.

## Interactions

| Needs from | What |
| --- | --- |
| Catalog | Terms open for admission |
| Advising | Programs available to apply to |
| Identity | Applicant and admissions staff accounts and roles |

| Provides to | What |
| --- | --- |
| Records | New student records for admitted, accepted applicants |
| Identity | Request to create or upgrade a user account to the student role |
| Advising | Intended program for new students |
| Finance | Application fees and deposits (stretch) |

## Open questions

- Is an applicant a user in Identity from the start, or only after admission?
- Is the student record created on admit or on acceptance?
- What happens to a waitlisted applicant when the term begins?
