import asyncio
import logging
import sys
import threading

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


class AgentRegistry:
    def __init__(self):
        self.agents = {}

    def register(self, key, agent):
        self.agents[key] = self.agents.get(key, []) + [agent]

    def get(self, key):
        return self.agents[key]

    def get_next_id(self, key):
        if key not in self.agents:
            return 0
        return len(self.agents[key])

    @property
    def all(self):
        all_agents = []
        for key in self.agents:
            all_agents.extend(self.agents[key])
        return all_agents


class Rakun:

    def __init__(self):
        self.loop = initialize_event_loop()
        self.registry = AgentRegistry()

    async def register(self, agent_class, domain=None):
        agent_id = f"{self.registry.get_next_id(agent_class)}"
        agent = agent_class(domain, id=agent_id)
        logger.debug(f"Registering agent {agent}")
        self.registry.register(agent_class, agent)

    async def start(self, driver=None):
        # await start_agents(self.registry.all)
        tasks = [self.loop.create_task(agent.start()) for agent in self.registry.all]
        await asyncio.tas
