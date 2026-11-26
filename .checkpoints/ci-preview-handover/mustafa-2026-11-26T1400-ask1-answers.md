# Mustafa's answers to ask 1: FF-REFETCH (a), UPLOAD-SOURCE (a), and what ask 2 must hold

_Copied by script from the chat's transcript (session_019YNMUUemXge56777MiUWFz), never retyped: the user message of 2026-11-26T13:04:35.110Z (uuid 235f7b30-0a58-4ed7-9e2f-88b44fcfa1ac), whole (1185 bytes, sha256 db5ad37dddf9d5b8693ee4c62437ef0f6a01de7e37803dd87b5237c29b499254). What it answers: the chat's ask 1 (question-2026-11-26-ask1.md at fb9b1a4): FF-REFETCH, fetch the same Firefox 161 file again for the trial; UPLOAD-SOURCE, may CI use actions/upload-artifact v7.0.2 pinned by cf430e030ddbb5b0abf93d22962f4752f3646cd9; each recommended (a)_

---
From Mustafa, 26 November, 14:00 Amsterdam (written with my Advisor).

1. FF-REFETCH: (a). The same file by the same route as for #71: firefox_161.0+build1-0ubuntu0.24.04.1~mt1_amd64.deb, 90,437,572 bytes, sha256 bd304b0b181c4fed449279e25d309075c3a2e85345e63d1118c5085cefcf1ed1. Size and checksum checked before use, unpacked without installing, deleted at this pull request's merge. If the archive no longer serves exactly that file, or Firefox needs anything more, stop and ask me.
2. UPLOAD-SOURCE: (a). actions/upload-artifact v7.0.2, pinned by the full commit cf430e030ddbb5b0abf93d22962f4752f3646cd9, for the hand-over step only. Nothing of it runs before my yes on ask 2's exact text.
3. Ask 2, in one message: every text with each changed file's sha256 after the change; the output line that ties v7.0.2 to that commit, confirmed by security-auditor; the guard line as its own yes or no; the hidden-files setting as its own yes or no; and security-auditor's answers on the steps that run after the network cut. My Advisor rebuilds the files and compares them before I answer.
4. Rule unchanged: git and gh always start their own command, with no other program in front of them.
