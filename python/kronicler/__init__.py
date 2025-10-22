from .kronicler import Database, database_init

from typing import Final
import time
from os import getenv
from starlette.requests import Request
from starlette.middleware.base import BaseHTTPMiddleware


# Create an ENV var for kronicler to be unset
KRONICLER_ENABLED = getenv("KRONICLER_ENABLED", "true").lower() in ("true", "1")

DB = Database(sync_consume=True)


def capture(func):
    if not KRONICLER_ENABLED:
        # Return the original function unchanged
        return func

    def wrapper(*args, **krawgs):
        # Use nano seconds because it's an int
        # def perf_counter_ns() -> int: ...
        start: int = time.perf_counter_ns()

        # TODO: Should I go through args manually here and only share ones that
        # are string, float, and int? This way I can actually store them
        # without having to do GIL in Rust, which would be very slow
        # https://github.com/JakeRoggenbuck/kronicler/issues/15
        #
        # for a in args:
        #   if isinstance(a, str):
        #       strings.append(a)
        value = func(*args, **krawgs)

        end: int = time.perf_counter_ns()

        DB.capture(func.__name__, args, start, end)

        return value

    return wrapper


def decorator_example(func):
    def wrapper():
        print("Kronicler start...")

        func()

        print("Kronicler end...")

    return wrapper


class KroniclerMiddleware(BaseHTTPMiddleware):
    def __init__(self, app):
        super().__init__(app)

    async def dispatch(self, request: Request, call_next):
        start: int = time.perf_counter_ns()
        response = await call_next(request)
        end: int = time.perf_counter_ns()

        # After call_next, the route has been matched
        route = request.scope.get("route")
        func_name = None

        if route:
            endpoint = getattr(route, "endpoint", None)
            if endpoint:
                func_name = endpoint.__name__

        # Fallback to path if no route found
        if not func_name:
            func_name = request.url.path

        DB.capture(func_name, [], start, end)
        return response


__all__: Final[list[str]] = ["kronicler"]
