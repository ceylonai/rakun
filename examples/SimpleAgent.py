import asyncio
import logging

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
        self.register_event_handler("after_start", self.after_start)
        # self.register_event_handler("after_start", self.after_start_async)

    def after_start(self):
        logger.debug(f"{self} after_start")
        while True:
            logger.info(f"Hello {self}")
            asyncio.sleep(1)

    async def after_start_async(self):
        logger.debug(f"{self} after_start_async")


async def main():
    rakun = rk.Rakun()

    await rakun.register(SimpleAgent)  # Here you can override them
    await rakun.register(SimpleAgent)  # Here you can override them
    await rakun.register(SimpleAgent)  # Here you can override them
    await rakun.start()


if __name__ == '__main__':
    asyncio.run(main())
