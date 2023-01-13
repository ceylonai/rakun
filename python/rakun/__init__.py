from .driver.com import MemoryDriver
from .rakun import *
from .features import *
from .plugins import *
from .agent import AgentWrapper

__doc__ = rakun.__doc__
if hasattr(rakun, "__all__"):
    __all__ = rakun.__all__


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
        self.agents[domain] = AgentWrapper(agent_impl, domain, features)

    async def start(self, driver=MemoryDriver):
        pass


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


def event(name):
    """
    Decorator for agent class
    :param name: name of the agent
    :return: agent class
    """

    def decorator(func):
        func.__name__ = name
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
