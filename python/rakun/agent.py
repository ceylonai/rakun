import inspect

from rakun import Agent, Event


class AgentWrapper(Agent):

    def __init__(self, impl_class, *args, **kwargs):
        event_functions = self.base_class
        for name, method in impl_class.__dict__.items():
            if callable(method) and hasattr(method, "event_type"):
                event_type = method.event_type
                print("event_type", event_type)
                event = Event(event_type, method)
                self.add_event(event_type, event)
