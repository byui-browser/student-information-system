# Records

## Purpose

Records owns the official academic record for each student: who they are,
their status at the institution, the grades they earned, and the transcript
that summarizes it all. It is the system of record for anything that would
appear on an official transcript.

## Slices

| Slice | Owns |
| --- | --- |
| Student profiles | Student demographics, contact info, status, class standing, academic standing, records holds |
| Grades and transcripts | Grade entry and approval, grade changes, GPA, term history, unofficial and official transcripts |

## Core concepts

- **Student:** student id, legal and preferred name, date of birth, contact information, status (active, inactive, graduated, withdrawn).
- **Class standing:** freshman, sophomore, junior, senior, based on earned credits.
- **Academic standing:** good standing, probation, suspension, based on GPA.
- **Grade:** a final grade for one student in one section, such as `A`, `B+`, `P`, `W`, `I`.
- **Grade change:** a correction to a submitted grade, with a reason and approval.
- **Transcript:** all terms, courses, grades, credits, and GPA for a student.

## Features

### Foundation

- Create and view student records.
- Update contact information and preferred name.
- Instructors submit final grades for a section roster.
- Calculate term GPA and cumulative GPA.
- View an unofficial transcript.

### Core

- Grade scale with grade points, including non-GPA grades (`P`, `W`, `I`, `AU`).
- Grade submission deadline per term (from Catalog).
- Grade change requests with registrar approval.
- Incomplete (`I`) grades that convert to a default grade after a deadline.
- Repeated courses: decide which attempt counts toward GPA.
- Class standing calculated from earned credits.
- Academic standing calculated at term end, which can place or lift a records hold.
- Student search for staff, by id or name.
- Official transcript request with a printable or PDF format.

### Stretch

- Transfer credit from other institutions.
- Dean's list and honors.
- Graduation application and degree conferral (with Advising).
- Privacy flag that restricts what can be shared (directory information).
- Data change history for every field.

## Business rules

- Student ids are unique and never reused.
- Only the instructor of record (or registrar) can submit grades for a section.
- Submitted grades can only change through a grade change request.
- GPA includes only grades that carry grade points.
- Sensitive data (date of birth, contact info) is visible only to the student and authorized staff.
- Records are never hard-deleted.

## Interactions

| Needs from | What |
| --- | --- |
| Enrollment | Final enrollments and withdrawals per term |
| Catalog | Course titles and credits, term dates |
| Admissions | New students created from admitted applicants |
| Identity | Instructor and staff roles |

| Provides to | What |
| --- | --- |
| Enrollment | Student status, completed courses and grades (for prerequisites) |
| Advising | Completed courses, grades, GPA for degree audit |
| Finance | Student name and status |
| Identity | Student id to link to a user account |

## Open questions

- Which grade scale, and which grades count toward GPA?
- On a repeated course, does the most recent or the highest attempt count?
- Who creates the student record: Admissions on admit, or Records on request?
