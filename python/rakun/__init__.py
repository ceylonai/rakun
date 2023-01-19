import asyncio
import logging
import sys

from .rakun import *

__doc__ = rakun.__doc__
if hasattr(rakun, "__all__"):
    __all__ = rakun.__all__

logger = logging.getLogger(__name__)


def initialize_event_loop():
    # platform_name = platform.machine()
    if sys.platform.startswith("win32") or sys.platform.startswith("linux-cross"):
        loop = asyncio.new_event_loop()
        asyncio.set_event_loop(loop)
        return loop
    else:
        # uv loop doesn't support windows or arm machines at the moment
        # but uv loop is much faster than native asyncio
        import uvloop

        uvloop.install()
        loop = uvloop.new_event_loop()
        asyncio.set_event_loop(loop)
        return loop


class Rakun:

    async def register(self, agent_class):
        agent = agent_class()
        logger.debug(f"Registering agent {agent}")
