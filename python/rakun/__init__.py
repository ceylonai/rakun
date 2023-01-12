from .driver.com import MemoryDriver
from .rakun import *
from .features import *
from .plugins import *

__doc__ = rakun.__doc__
if hasattr(rakun, "__all__"):
    __all__ = rakun.__all__


def Agent(name, domain, features=None):
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


def behave(name):
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
    :param name: name of the agent
    :return: agent class
    """

    def decorator(func):
        func.__sender__ = sender
        func.__subject__ = subject
        return func

    return decorator


async def register(agent, domain, features=None):
    """
    Register agent
    :param agent: agent class
    :param domain: domain of the agent
    :param features: list of features
    :return: agent class
    """
    pass


async def start(driver=MemoryDriver):
    pass
