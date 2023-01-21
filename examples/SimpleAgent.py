import asyncio
import logging
import threading
import time

import rakun as rk

logger = logging.getLogger(__name__)
log_level = "DEBUG"  # self.log_level if self.log_level else log_level
logging.basicConfig(level=log_level, format='%(asctime)s %(message)s')


class SimpleAgent(rk.Agent):
    def __new__(self, id=None):
        return super().__new__(self, domain="simpleagent1@rk", id=id)

    def __init__(self, *args, **kwargs):
        super().__init__()
        logger.debug("SimpleAgent init")
        # self.register_event_handler("after_start", self.after_start)
        self.register_event_handler("after_start", self.after_start_async)
        self.register_event_handler("on_message", self.on_message)

    async def after_start_async(self):
        logger.debug(f"{self} after_start_async")
        while True:
            await self.send({"data": "hello"})
            await asyncio.sleep(0.1)

    async def on_message(self):
        print(f"on_message {self}")


async def main():
    rakun = rk.Rakun()

    # for i in range(50000):
    await asyncio.gather(*[rakun.register(SimpleAgent) for i in range(2)])
    await rakun.start()


if __name__ == '__main__':
    asyncio.run(main())
