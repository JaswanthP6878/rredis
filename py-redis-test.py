import redis
from redis.connection import Connection

# debugiing Logging connection class for logging the wire commands
class LoggingConnection(Connection) :
    def send_packed_command(self, command, check_health=True):
        print(f"wire message being sent is : {command}")
        return super().send_packed_command(command, check_health)

pool = redis.ConnectionPool(connection_class=LoggingConnection, host= "localhost", port=6379)
r = redis.Redis(connection_pool=pool)
print(f'connection is made')

print(r.ping()); # must respond with pong
print(f'ping message is sent')
response = r.set(name="foo", value=3)
response = r.get(name="foo")
print(response)
