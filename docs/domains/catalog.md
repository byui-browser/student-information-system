# Catalog

## Purpose

Catalog is the source of truth for **what is offered**: courses, their
prerequisites, academic terms, and the scheduled sections of each course. It
does **not** decide who is in a section; that is Enrollment.

## Slices

| Slice | Owns |
| --- | --- |
| Courses | Course definitions, subjects, credits, descriptions, prerequisites, corequisites, course search |
| Terms and sections | Academic terms and key dates, sections, meeting times, rooms, instructors, capacity, schedule search |

## Core concepts

- **Subject:** a group of courses, such as `CS` or `MATH`.
- **Course:** a catalog entry such as `CS 101 Introduction to Programming`, with credits, description, and status (active, retired).
- **Prerequisite:** a course (or set of courses) that must be completed first, optionally with a minimum grade.
- **Term:** an academic period (e.g. Spring 2027) with key dates: registration open, classes start, add/drop deadline, withdraw deadline, classes end, grades due.
- **Section:** one offering of a course in a term, with section number, instructor(s), meeting times, location, delivery mode, and capacity.

## Features

### Foundation

- Create, view, update, and retire courses.
- Create terms with their key dates.
- Create sections of a course for a term, with capacity.
- List and search courses by subject, code, or keyword.
- List sections for a term, filtered by course or subject.
- Seed data: a realistic set of subjects, courses, and one or two terms of sections.

### Core

- Prerequisites and corequisites, including "A or B" and minimum grade.
- Meeting times and rooms, with conflict detection (room double-booked, instructor double-booked).
- Assign instructors to sections.
- Report seats taken and seats remaining (seat counts come from Enrollment).
- Public course search page that works without signing in.
- Section status: planned, open, closed, cancelled.
- Cross-listed courses (one section offered under two subject codes).

### Stretch

- Room inventory with capacity and features (lab, accessible, projector).
- Copy all sections from one term to the next.
- Course change history, with an effective term for each change.
- Course evaluations or syllabus upload.

## Business rules

- Course codes are unique and never reused for a different course.
- Retired courses keep their history but cannot get new sections.
- A section's capacity cannot drop below the number of students already enrolled.
- Prerequisite chains must not form a cycle.
- Term dates must be in order (registration opens before classes start, and so on).
- Cancelling a section that has enrollments must notify Enrollment.

## Interactions

| Needs from | What |
| --- | --- |
| Identity | Instructor user ids |
| Enrollment | Seat counts per section |

| Provides to | What |
| --- | --- |
| Enrollment | Sections, capacity, meeting times, prerequisites, term dates |
| Records | Course titles and credits for transcripts |
| Advising | Courses for program requirements |
| Finance | Credits and course fees for billing |

## Open questions

- Does Catalog store seat counts, or always ask Enrollment?
- How are prerequisites represented so both Enrollment and Advising can evaluate them?
- Who may edit the catalog: registrar only, or department staff too?
