import asyncio
import rakun as rk
from rakun import features as rk_features
from rakun.driver.com import MemoryDriver


@rk.agent(name="SimpleAgent",
          domain="simpleagent@rk",
          features=[rk_features.Metric, rk_features.Performance, rk_features.Debug])  # port domain not required
class SimpleAgent2:
    pass


@rk.agent(name="SimpleAgent",
          domain="simpleagent@rk",
          features=[rk_features.Metric, rk_features.Performance, rk_features.Debug])  # port domain not required
class SimpleAgent:
    def __init__(self):
        print("SimpleAgent init")

    @rk.event("before_start")
    async def __before_start__(self):
        pass

    @rk.event("after_start")
    async def __after_start__(self):
        pass

    @rk.event("before_finish")
    async def __after_start__(self):
        pass

    @rk.event("after_finish")
    async def __after_start__(self):
        pass

    @rk.event("forever")  # forever, periodic, on_event, on_message, on_start, on_stop
    async def run(self):
        message = rk.Message()
        message.topic = "hello"
        message.to = "simpleagent@rk"
        message.body = {
            "message": "Hello World"
        }
        self.core.send_message(message)

    @rk.process(sender="*")
    async def process(self, message, sender):  # here sender must be available
        print(message)

    @rk.process(sender="world")
    async def process_world_message(self, message):  # (self, message, sender): same work but no need
        print(message)

    @rk.process(sender="world", subject="hello")
    async def process_world_message(self, message):  # (self, message, sender): same work but no need
        print(message)

    @rk.process(sender="world", subject="*hello*")
    async def process_world_message(self, message):  # (self, message, sender): same work but no need
        print(message)


async def main():
    rakun = rk.Rakun()

    await rakun.register(SimpleAgent, domain="simpleagent@rk",
                         features=[rk.features.Metric, rk.features.Performance,
                                   rk.features.Debug])  # Here you can override them

    await rakun.register(SimpleAgent2)  # Here you can override them
    await rakun.start(driver=MemoryDriver)


if __name__ == '__main__':
    asyncio.run(main())
