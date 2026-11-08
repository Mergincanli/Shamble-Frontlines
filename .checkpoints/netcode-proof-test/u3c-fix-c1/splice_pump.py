"""Replaces delay.rs's poll_shutdown and Pump (from poll_shutdown to the tests' #[path]) with pump_new.rs.txt."""
import pathlib
import sys

here = pathlib.Path(__file__).parent
target = pathlib.Path(sys.argv[1])
s = target.read_text()
start = s.index("    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {")
end = s.index('#[cfg(test)]\n#[path = "delay_tests.rs"]')
new = (here / "pump_new.rs.txt").read_text()
target.write_text(s[:start] + new + s[end:])
print("spliced", end - start, "->", len(new))
