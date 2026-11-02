SUBS = [
("| for a same-repository pull request, the link exists within 5 minutes after the build job ends, median of 5 runs |",
 "| for a same-repository pull request, the link exists within 5 minutes after the build job ends, or, while `preview`'s required reviewer holds (part 10, ADR 0006's interim for B9), after the reviewer's approval, median of 5 runs |"),
("| the build job's end and the link comment's time, from GitHub's records |",
 "| the build job's end (or the approval's time) and the link comment's time, from GitHub's records |"),
]
