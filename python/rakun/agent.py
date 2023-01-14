import inspect

from rakun import Agent, Event


class AgentWrapper(Agent):

    def __init__(self, impl_class, *args, **kwargs):
        event_functions = self.base_class

