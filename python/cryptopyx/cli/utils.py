import threading
import time
from collections.abc import Callable
from typing import TypeVar

# Type variables for generic function
# T is the return type, U is the argument type (str for now)
T = TypeVar('T')
U = TypeVar('U', bound=str)


def calc_loading(func: Callable[[U], T], *args: U, **kwargs: U) -> T:
    """Perform calculation with loading animation."""
    # Create thread and event to manage loading animation
    completed = threading.Event()
    loading_thread = threading.Thread(
        target=loading_animation, args=('Calculating...', completed)
    )
    # Start loading animation thread
    loading_thread.start()
    # Perform the calculation
    try:
        result = func(*args, **kwargs)
    except Exception as e:
        completed.set()
        loading_thread.join()
        raise e
    # Signal completion and wait for loading thread to finish
    completed.set()
    loading_thread.join()
    return result


def loading_animation(message: str, done: threading.Event) -> None:
    """Display a loading animation while `done` is not set."""
    # Frames for the loading animation
    frames: list[str] = ['⣷', '⣯', '⣟', '⡿', '⢿', '⣻', '⣽', '⣾']
    # Infinite loop until done is set
    while True:
        # Cycle through frames
        for frame in frames:
            print(f'\r{frame} {message}', end='', flush=True)
            time.sleep(0.025)
        if done.is_set():
            break
    # Clear the loading message and indicate completion
    print('\rDone!', end='', flush=True)
    time.sleep(0.05)
    print(f'\r{" " * (3 + len(message))}\r', end='', flush=True)
