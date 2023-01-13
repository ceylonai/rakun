from rakun import MemoryDriver


class Rakun:

    async def register(self, agent, domain, features=None):
        """
        Register agent
        :param agent: agent class
        :param domain: domain of the agent
        :param features: list of features
        :return: agent class
        """
        pass

    async def start(self, driver=MemoryDriver):
        pass
