#!/bin/bash

# echo "*2\r\n$4\r\nECHO\r\n$3\r\nhey\r\n" | nc localhost 6379 &
# echo "*1\r\n$4\r\nPING\r\n" | nc localhost 6379 &

# echo -ne '*1\r\n$4\r\nping\r\n' | nc localhost 6379

# (printf "*2\r\n$4\r\nECHO\r\n$3\r\nhey\r\n\r\n";) | nc localhost 6379 &
# (printf "*1\r\n$4\r\nPING\r\n";) | nc localhost 6379  & 
# (printf "*3\r\n$3\r\nSET\r\n$3\r\nFOO\r\n$\r\nEXTRA\r\n";) | nc localhost 6379
# (printf "*3\r\n$3\r\nSET\r\n$3\r\nFAR\r\n$\r\nNEAR\r\n";) | nc localhost 6379

# (printf "*2\r\n$3\r\nGET\r\n$3\r\nFOO\r\n";) | nc localhost 6379  


# (printf "*3\r\n\$3\r\nSET\r\n\$3\r\nFOO\r\n\$3\r\nBAR\r\n\$2\r\nPx\r\n\$3\r\n100\r\n";) | nc localhost 6379

# sleep 0.2 && (printf "*2\r\n$3\r\nGET\r\n$3\r\nFOO\r\n";) | nc localhost 6379  

# (printf "*3\r\n\$3\r\nSET\r\n\$3\r\nFOO\r\n\$3\r\nLoL\r\n\$2\r\nPx\r\n\$3\r\n100\r\n";) | nc localhost 6379
# (printf "*2\r\n$3\r\nGET\r\n$3\r\nFOO\r\n";) | nc localhost 6379 
# (printf "*2\r\n$3\r\nCONFIG\r\n$3\r\nGET\r\n$4\r\ndir\r\n";) | nc localhost 6379  

# (printf "*2\r\n$4\r\nKEYS\r\n$2\r\nF*\r\n";) | nc localhost 6379  


### KEYS Test
# (printf "*3\r\n$3\r\nSET\r\n$3\r\nFOO\r\n$\r\nEXTRA\r\n";) | nc localhost 6379
# (printf "*3\r\n$3\r\nSET\r\n$3\r\nFAR\r\n$\r\nNEAR\r\n";) | nc localhost 6379
# (printf "*2\r\n$3\r\nGET\r\n$3\r\nFOO\r\n";) | nc localhost 6379 
# (printf "*2\r\n$4\r\nKEYS\r\n$2\r\nF*\r\n;) | nc localhost 6379  
###


### SAVE TEST
# (printf "*3\r\n$3\r\nSET\r\n$3\r\nFOO\r\n$\r\nEXTRA\r\n";) | nc localhost 5000
# (printf "*3\r\n$3\r\nSET\r\n$3\r\nFAR\r\n$\r\nNEAR\r\n";) | nc localhost 5000
# (printf "*1\r\n$4\r\nSAVE\r\n";) | nc localhost 5000
### 


### test replication info
# (printf "*3\r\n$3\r\nset\r\n$3\r\nfoo\r\n$\r\nextra\r\n";) | nc localhost 5000
# (printf "*2\r\n$4\r\ninfo\r\n$5\r\nreplication\r\n";) | nc localhost 5000
### 


# test replication info
(printf "*3\r\n$3\r\nset\r\n$3\r\nfoo\r\n$\r\nextra\r\n";) | nc localhost 6379
# (printf "*2\r\n$4\r\ninfo\r\n$5\r\nreplication\r\n";) | nc localhost 5000







