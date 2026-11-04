# Security policy

## Supported versions

Only the latest release gets security fixes: the desktop app and the Docker image
(`:latest`). Please check that the problem still happens on it before reporting. The
desktop app offers new versions in **Settings › About**; on Docker, run
`docker compose pull`, then `docker compose up -d`.

## Reporting a vulnerability

Please do not report a vulnerability in a public issue, discussion or pull request.

Report it privately with GitHub's private vulnerability reporting: open the repository's
**Security** tab and click **Report a vulnerability**. Only you and the maintainers see
the report, and it stays private until its advisory is published.

## What to include

- The version (**Settings › About**, or the first line of the logs copied from the
  **Technical logs** page) and where it runs: the desktop app and its system, or the
  Docker image, over plain HTTP or behind an HTTPS reverse proxy (and which proxy).
- The part concerned, for example the dashboard sign-in and sessions, the HTTP API, the
  token storage, the updates or the notifications.
- The steps to reproduce it, or a proof of concept, and what an attacker gains.
- Logs when they help: **Technical logs › Copy for a bug report** gives a redacted copy.

Never send real tokens, passwords, the `master.key` file or the database.

## After a report

The report is discussed in its private advisory. Once a fix is released, the advisory
is published, with credit to you if you wish.

## Scope

This repository's code: the desktop app, the server and its dashboard, the Docker image,
and the release and update process. Problems in Twitch itself or in other services the
app talks to belong to those services.
