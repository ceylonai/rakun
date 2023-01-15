import rakun


class MemoryDriver(rakun.Driver):

    def __new__(cls):
        return super().__new__(cls, name="MemoryDriver")
