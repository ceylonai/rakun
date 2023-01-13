from .driver.com import MemoryDriver
from .rakun import *
from .features import *
from .plugins import *
from .rakun_server import *

__doc__ = rakun.__doc__
if hasattr(rakun, "__all__"):
    __all__ = rakun.__all__


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
