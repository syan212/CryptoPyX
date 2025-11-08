import sys
import threading
import time
from collections.abc import Callable


def calc_loading(func: Callable, *args: object, **kwargs: dict[str, object]) -> object:
    """Perform calculation with loading animation."""
    completed = threading.Event()
    loading_thread = threading.Thread(
        target=loading_animation, args=('Calculating...', completed)
    )
    loading_thread.start()
    try:
        result = func(*args, **kwargs)
    except Exception as e:
        completed.set()
        loading_thread.join()
        raise e

    completed.set()
    loading_thread.join()
    return result


def loading_animation(message: str, done: threading.Event) -> None:
    """Display a loading animation while `done` is not set."""
    frames: list[str] = ['⣷', '⣯', '⣟', '⡿', '⢿', '⣻', '⣽', '⣾']
    while True:
        for frame in frames:
            if done.is_set():
                break
            sys.stdout.write(f'\r{frame} {message}')
            sys.stdout.flush()
            time.sleep(0.05)
        if done.is_set():
            break
    sys.stdout.write('\rDone!')
    sys.stdout.flush()
    time.sleep(0.05)
    sys.stdout.write(f'\r{" " * (3 + len(message))}\r')
    sys.stdout.flush()
