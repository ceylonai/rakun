import asyncio
import logging
import time

import rakun as rk

logger = logging.getLogger(__name__)
log_level = "INFO"  # "DEBUG"  # self.log_level if self.log_level else log_level
logging.basicConfig(level=log_level, format='%(asctime)s %(message)s')


class SimpleAgent(rk.Agent):
    counter = 1
    start_time = time.time_ns()

    def __init__(self, *args, **kwargs):
        super().__init__()
        logger.debug("SimpleAgent init")
        self.register_event_handler("after_start", self.after_start_async)
        self.register_event_handler("on_message", self.on_message)

    async def after_start_async(self):
        logger.debug(f"{self} after_start_async")
        while True:
            await self.send({"time": f"{time.time_ns()}"})
            self.counter += 1
            await asyncio.sleep(0.0001)

    async def on_message(self, message):
        if self.counter % 200 == 0:
            logger.debug(f"on_message {self} {message}")
            received_time = int(message["time"])
            diff = (time.time_ns() - received_time) / 1e9
            rate = 1 / diff
            print(f"Rate {self} {rate:f} msg/s")

        if self.counter % 1000 == 0:
            await self.exit()


async def main():
    rakun = rk.Rakun()

    # for i in range(50000):
    await asyncio.gather(*[rakun.register(SimpleAgent, "simpleAgent@rk") for i in range(2)])
    await rakun.start()


if __name__ == '__main__':
    asyncio.run(main())
