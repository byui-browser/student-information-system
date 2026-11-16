# Advising

## Purpose

Advising helps students graduate on time. It defines degree programs and
their requirements, runs degree audits that show what a student has finished
and what remains, and supports the advisor relationship (assignments, notes,
holds, and plans).

## Slices

| Slice | Owns |
| --- | --- |
| Programs and requirements | Majors, minors, certificates, requirement rules, catalog year, a student's declared programs |
| Degree audit and advisor tools | Degree audit, what-if audit, advisor assignments, advising notes, advising holds, academic plans |

## Core concepts

- **Program:** a major, minor, or certificate, with a required number of credits.
- **Requirement:** a rule within a program, such as "complete CS 101", "complete 3 of these 5 courses", "12 credits of upper-division CS", or "minimum 2.0 GPA in major".
- **Catalog year:** the version of a program's requirements a student follows.
- **Declaration:** a student's chosen program(s).
- **Degree audit:** completed, in-progress, and remaining requirements for a student.
- **Advisor assignment:** which advisor works with which student.
- **Advising note:** a record of an advising conversation.
- **Academic plan:** courses a student intends to take in future terms.

## Features

### Foundation

- Create programs with a list of required courses.
- Declare a program for a student.
- Run a degree audit: completed and remaining required courses.
- Assign advisors to students.
- Advisor view: list of assigned students.

### Core

- Rich requirement rules: choose N of a list, minimum credits from a subject or level, minimum grade, minimum GPA.
- Count in-progress courses (from Enrollment) as "in progress" in the audit.
- Catalog year: audit against the requirements in effect when the student started.
- What-if audit: "what would my audit look like if I switched to this major?"
- Advising notes, visible to advisors and optionally to the student.
- Advising holds that require a meeting before registration.
- Course substitutions and waivers approved by an advisor.

### Stretch

- Academic plan builder that checks prerequisites and term offerings.
- Graduation readiness check (feeds Records' degree conferral).
- Advisor appointment scheduling.
- Early alerts: instructors flag students who are struggling.
- Double majors and courses that count toward more than one program.

## Business rules

- A course counts toward a requirement only with a passing grade at or above the requirement's minimum.
- A student follows exactly one catalog year per program.
- Only the student's assigned advisor (or an admin) can place or release an advising hold.
- Substitutions and waivers must record who approved them and why.
- Advising notes are private to advisors unless marked shareable.

## Interactions

| Needs from | What |
| --- | --- |
| Catalog | Courses, credits, subjects, levels |
| Records | Completed courses, grades, GPA, transfer credit |
| Enrollment | Current and planned enrollments |
| Admissions | Intended program of newly admitted students |
| Identity | Advisor and student roles |

| Provides to | What |
| --- | --- |
| Enrollment | Advising holds, declared program (for priority rules) |
| Records | Graduation readiness |
| Finance | Advising holds (if holds are centralized) |
| Students (clients) | Degree audit and academic plan |

## Open questions

- How are requirement rules stored so they are flexible but still testable?
- How do transfer credits and substitutions appear in the audit?
- Does declaring a program require advisor approval?
