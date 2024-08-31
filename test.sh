#!/bin/bash

# echo "*2\r\n$4\r\nECHO\r\n$3\r\nhey\r\n" | nc localhost 6379 &
# echo "*1\r\n$4\r\nPING\r\n" | nc localhost 6379 &

(printf "*2\r\n$4\r\nECHO\r\n$3\r\nhey\r\n";) | nc localhost 6379
(printf "*1\r\n$4\r\nPING\r\n";) | nc localhost 6379