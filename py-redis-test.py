import redis
from redis.connection import Connection

# debugiing Logging connection class for logging the wire commands
class LoggingConnection(Connection) :
    def send_packed_command(self, command, check_health=True):
        print(f"wire message being sent is : {command}")
        return super().send_packed_command(command, check_health)

pool = redis.ConnectionPool(connection_class=LoggingConnection, host= "localhost", port=6379)
r = redis.Redis(connection_pool=pool)

print(r.ping()); # must respond with pong
response = r.set(name="foo", value=3)
print(type(response))
# print(type(r.get(name="foo")))
response = r.get(name="foo")
