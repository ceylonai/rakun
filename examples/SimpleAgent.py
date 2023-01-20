import asyncio
import logging

import rakun as rk

logger = logging.getLogger(__name__)
# log_level = "WARN"

# if self.dev:
#     log_level = "DEBUG"

log_level = "DEBUG"  # self.log_level if self.log_level else log_level
logging.basicConfig(level=log_level, format='%(asctime)s %(message)s')


# @rk.agent(name="SimpleAgent",
#           domain="simpleagent1@rk",
#           features=[rk_features.Metric, rk_features.Performance, rk_features.Debug])  # port domain not required
class SimpleAgent(rk.Agent):
    def __new__(self, id=None):
        return super().__new__(self, domain="simpleagent1@rk", id=id)

    def __init__(self, *args, **kwargs):
        super().__init__()
        logger.debug("SimpleAgent init")
        self.register_event_handler("after_start", self.after_start)
        self.register_event_handler("after_start", self.after_start_async)

    # # @rk.event("after_agent_start")
    def after_start(self):
        logger.debug(f"{self} after_agent_start")

    # # @rk.event("after_agent_start")
    async def after_start_async(self):
        logger.debug(f"{self} after_agent_start")
    #
    # # @rk.event("before_agent_stop")
    # async def __before_agent_finish__(self):
    #     print("SimpleAgent before_agent_stop")
    #
    # # @rk.event("forever")  # forever, periodic, on_event, on_message, on_start, on_stop
    # async def run(self):
    #     while True:
    #         print("SimpleAgent run")
    #         await asyncio.sleep(1)
    #     # message = rk.Message()
    #     # message.topic = "hello"
    #     # message.to = "simpleagent@rk"
    #     # message.body = {
    #     #     "message": "Hello World"
    #     # }
    #     # self.core.send_message(message)
    #
    # # @rk.process(sender="*")
    # async def process(self, message, sender):  # here sender must be available
    #     print(message)
    #
    # # @rk.process(sender="world")
    # async def process_world_message(self, message):  # (self, message, sender): same work but no need
    #     print(message)
    #
    # # @rk.process(sender="world", subject="hello")
    # async def process_world_message(self, message):  # (self, message, sender): same work but no need
    #     print(message)
    #
    # # @rk.process(sender="world", subject="*hello*")
    # async def process_world_message(self, message):  # (self, message, sender): same work but no need
    #     print(message)


async def main():
    rakun = rk.Rakun()

    await rakun.register(SimpleAgent)  # Here you can override them
    await rakun.register(SimpleAgent)  # Here you can override them
    await rakun.register(SimpleAgent)  # Here you can override them
    await rakun.start()

    # await rakun.register(SimpleAgent2)  # Here you can override them
    # await rakun.start(driver=MemoryDriver)


if __name__ == '__main__':
    asyncio.run(main())
