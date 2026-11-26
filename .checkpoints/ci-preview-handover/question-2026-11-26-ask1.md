# Ask 1 for Mustafa, 2026-11-26: FF-REFETCH and UPLOAD-SOURCE

The advisor agent's read of plan revision 1: REVISE (six changes, all in the plan's text; revision 2 at e59b0c2, its
re-read running). It ruled that these two questions may go to him now.

## FF-REFETCH: may the chat fetch the Firefox file for the trial again?
Why: the replay test has never run in Firefox; the file he allowed for #71's trial (TRIAL-DOWNLOAD (a)) was deleted at
#71's merge, as his rule said, and his rule 3 asks for a new yes before any download is used.
The file: `firefox_161.0+build1-0ubuntu0.24.04.1~mt1_amd64.deb`, from
`https://ppa.launchpadcontent.net/mozillateam/ppa/ubuntu/pool/main/f/firefox/firefox_161.0+build1-0ubuntu0.24.04.1~mt1_amd64.deb`,
90,437,572 bytes, sha256 bd304b0b181c4fed449279e25d309075c3a2e85345e63d1118c5085cefcf1ed1 (the same values as before).
The route: https into a new folder under `.gate/`, size and sha256 checked before use, `dpkg-deb -x`, no apt, no root,
no install script; deleted at this pull request's merge. If the archive no longer serves that exact file, the chat
stops and tells him. Without the two graphics library files of #71 (the replay test draws nothing); if Firefox needs
them, that is a stop and a question.
Options: (a) yes; (b) no, the first GitHub run is the replay test's only trial. Recommended: (a).

## UPLOAD-SOURCE: may CI use the new outside tool `actions/upload-artifact`?
Why: lane 2's step needs it, and his opening rule 3 ends "No other new package or tool"; his block of 13:20 asks for
the step that needs it, so the two of his words meet here and only he can settle it. The repository has never used it
(today's workflows use only checkout, setup-node and cache).
What it is: GitHub's own upload action, `https://github.com/actions/upload-artifact`, version v7.0.2, pinned by full
commit cf430e030ddbb5b0abf93d22962f4752f3646cd9 (read from the action's own repository today with `git ls-remote`:
v7 and v7.0.2 both at that commit; read again by the advisor agent). Read, not run, at that commit: it follows symbolic
links (so the proposed guard line matters), it needs `include-hidden-files: true` for a folder under `.gate/`, and it
takes no token input (it uses the runner's own short-lived upload key, so the job's token stays where it is).
Options: (a) yes, at that pin; (b) no: no hand-over step, and lane 2's preview waits for another route. Recommended:
(a). Nothing of it runs before the CI write, which still comes to him as ask 2 with its exact text.
