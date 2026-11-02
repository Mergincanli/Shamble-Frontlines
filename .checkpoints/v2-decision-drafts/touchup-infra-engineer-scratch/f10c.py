ID = 'F10 (OQ-0006-14, why it matters)'
TITLE = 'OQ-0006-14: the lobby routes by build, the reload no longer loops'
START, END = 918, 918
AFTER = """    client with `JoinRejected`, and the lobby sends it only to rooms of its build (ADR 0001 part 11), so from the"""
WHY = """F10 holds (OQ-0006-14). "its reload fetches the same new bundle (ADR 0001's failure table)" (0006:918) is the old
row; the failure table now reloads at most once and only for a newer build (0001:719), and the lobby sends a client
only to rooms of its build (0001:495-496, 1628-1629). "sends" for the verifier's "can route": fewer words, same claim."""
