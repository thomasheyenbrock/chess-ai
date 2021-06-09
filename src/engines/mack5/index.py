import subprocess
from typing import Optional, Tuple

nim = subprocess.Popen(
    "./engines/mack5/mcts",
    universal_newlines=True,
    stdin=subprocess.PIPE,
    stdout=subprocess.PIPE,
)
# readiness probe
nim.stdout.readline()

current_id = None


def mack5(id: str, fen: str) -> Tuple[int, int, Optional[str]]:
    global current_id
    if current_id != None and id != current_id:
        nim.stdin.write("reset\n")
        nim.stdin.flush()
    current_id = id

    nim.stdin.write(f"{fen}\n")
    nim.stdin.flush()
    from_square = int(nim.stdout.readline().strip())
    to_square = int(nim.stdout.readline().strip())
    is_promoting_to = nim.stdout.readline().strip()
    return (
        from_square,
        to_square,
        None if is_promoting_to == "0" else is_promoting_to,
    )
