import asyncio
import logging
import sys

from .rakun import *
from .features import *
from .plugins import *
from .agent import AgentWrapper
from .driver.com import MemoryDriver

__doc__ = rakun.__doc__
if hasattr(rakun, "__all__"):
    __all__ = rakun.__all__


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


def get_events(cls):
    events = {}
    for name, method in cls.__dict__.items():
        if callable(method) and hasattr(method, "event_type"):
            event_type = method.event_type
            print("event_type", event_type, method)
            event = Event(event_type, method)
            if event_type in events:
                events[event_type].append(event)
            else:
                events[event_type] = [event]
    return events


class Rakun:
    agents = {}

    async def register(self, agent_impl, domain=None, features=None):
        """
        Register agent
        :param agent_impl:
        :param domain: domain of the agent
        :param features: list of features
        :return: agent class
        """
        initialize_event_loop()
        self.agents[domain] = AgentWrapper(agent_impl, domain, features, get_events(agent_impl))

    async def start(self, driver=MemoryDriver):
        driver = driver()
        for agent in self.agents.values():
            await agent.start(driver)


def agent(domain=None, name=None, features=None):
    """
    Decorator for agent class
    :param name: name of the agent
    :param domain: domain of the agent
    :param port: port of the agent
    :param features: list of features
    :return: agent class
    """

    def decorator(cls):
        cls.__name__ = name
        cls.__domain__ = domain
        cls.__features__ = features
        return cls

    return decorator


def event(event_type):
    """
    Decorator for agent class
    :param event_type: type of the registered event
    :return: agent class
    """

    def decorator(func):
        # func = Event(func, event_type)
        func.event_type = event_type
        return func

    return decorator


def process(sender, subject=None):
    """
    Decorator for agent class
    :param sender:
    :param subject:
    :return: agent class
    """

    def decorator(func):
        func.__sender__ = sender
        func.__subject__ = subject
        return func

    return decorator
