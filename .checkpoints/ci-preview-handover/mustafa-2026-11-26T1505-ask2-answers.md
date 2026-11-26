# Mustafa's answers to ask 2: CI-WRITE-3 (a), GUARD-STEP (a), HIDDEN-FILES (b)

_Copied by script from the chat's transcript (session_019YNMUUemXge56777MiUWFz), never retyped: the user message of 2026-11-26T14:08:56.732Z (uuid 8e3f694b-4053-4ce3-a2fd-d0dcff40d13e), whole (1305 bytes, sha256 117acbaf961449dcf96cb87012776140309726104b9414d6bf9f8df882a8b145). What it answers: the chat's ask 2 (question-2026-11-26-ask2.md at 3e3cf08): CI-WRITE-3, write the workflow and the test configuration in the recommended form; GUARD-STEP, keep the check before the upload; HIDDEN-FILES, include-hidden-files on or off; recommended (a), (a), (b)_

---
From Mustafa, 26 November, 15:05 Amsterdam (written with my Advisor).

1. CI-WRITE-3: (a). Write the two texts in the recommended form. My Advisor rebuilt both from main and your saved diffs at 3e3cf08 and got your values: .github/workflows/toolchain.yml d017b83b82d412befb1cbb983a93215440e2b0e1608d2c8c749445db53e87fd8, packages/e2e/playwright.config.ts 35abfd54fddb43d7bf8077b281c153c8625c22d863d04922afd0b7b1ecfe80c8. After writing, show both files' sha256. Any other value is a stop.
2. GUARD-STEP: (a). Keep the check before the upload.
3. HIDDEN-FILES: (b). Off.
4. The upload action stays at the pin cf430e030ddbb5b0abf93d22962f4752f3646cd9.
5. Actions permissions: I do not look it up. The first run on the draft pull request is the test. If it fails at its start because the action is not allowed, stop and tell me which setting to change.
6. CUT-ESCAPE stays on record as accepted. Put its later fix (the cut's code as its own throwaway user) into STATUS as a follow-up.
7. Then as before: the docs pass, the report, and the pull request's title and text in your notes. My Advisor opens the draft and watches the first run. A red on the new steps or on Firefox's replay test comes to me first.
8. Rule unchanged: git and gh always start their own command, with no other program in front of them.
