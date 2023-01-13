import inspect

from rakun import Agent


class AgentWrapper(Agent):

    def __created__(self):
        super().__created__()
        print("after start", self.domain)
