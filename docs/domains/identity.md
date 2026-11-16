# Identity

## Purpose

Identity answers "who is this, and what are they allowed to do?" It manages
user accounts, roles, and authentication for every person who uses the
system: students, instructors, advisors, registrar staff, finance staff,
admissions staff, and administrators. It does **not** own academic or
personal records; those belong to Records.

## Slices

| Slice | Owns |
| --- | --- |
| Accounts and roles | Users, roles, permissions, account lifecycle, user administration |
| Authentication and sessions | Sign in, sign out, tokens, sessions, password reset, multi-factor |

## Core concepts

- **User:** a person who can sign in. Has a username, email, display name, and status (active, locked, deactivated).
- **Role:** student, instructor, advisor, registrar, finance, admissions, admin. A user may hold more than one.
- **Permission:** a specific action, such as `grades:write` or `holds:release`. Roles grant permissions.
- **Session / token:** proof that a request comes from a signed-in user.
- **Principal:** the authenticated caller, as seen by other domains.

## Features

### Foundation

- Create, view, update, and deactivate user accounts.
- Assign and remove roles.
- Sign in with username and password; receive a token.
- Validate a token and return the principal (used by Platform's auth middleware).
- Sign out and invalidate the session.
- "Who am I" endpoint for clients.
- Seed accounts for each role for local development.

### Core

- Passwords stored with a modern password hashing algorithm, never in plain text.
- Password reset flow (request, token, reset).
- Account lockout after repeated failed sign-ins.
- Permission checks that other domains can call ("can this principal do X?").
- Link a user account to a student record (Records) or staff profile.
- Admin screen to search users and manage roles.
- Session expiry and refresh.

### Stretch

- Multi-factor authentication (TOTP).
- Single sign-on with an external identity provider (OpenID Connect).
- Proxy access: a student grants a parent or guardian limited read access.
- Impersonation for support staff, fully audited.
- Sign-in history visible to the user.

## Business rules

- Usernames and emails are unique.
- Deactivated or locked users cannot sign in, and their existing sessions stop working.
- Only admins can grant the admin role.
- A user cannot remove their own admin role.
- Every role change is audited.
- Error messages on failed sign-in must not reveal whether the username exists.

## Interactions

| Needs from | What |
| --- | --- |
| Records | Student id to link to a student user account |
| Admissions | Request to create an account when an applicant is admitted |

| Provides to | What |
| --- | --- |
| Platform | Token validation for auth middleware |
| Every domain | Principal (user id and roles) and permission checks |

## Open questions

- Opaque session tokens or signed tokens (JWT)? How long do they live?
- Are permissions fixed per role, or configurable by admins?
- Does Identity create the account on admission, or does Admissions ask for it?
