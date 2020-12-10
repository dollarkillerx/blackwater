# blackwater
Blaskwate rust port scanner tool

Scan all ports in one second, basically no missing
### Install
``

### Usage
`blackwater -h`

### speed
Amazing speed Not a single port is missed
``` 
ubuntu@ubuntu:~/$ time blackwater -i 110.242.68.3 -p1-65535

 _      _
| |    | |
| |_   | |   __
| | |  | |  |  |
| _ |  |_|  |  |
Black Water
Asynchronous Port Scanner written in rust
https://github.com/dollarkillerx/blackwater

110.242.68.3:80
110.242.68.3:443
110.242.68.3:2000
110.242.68.3:5060

real    0m1.637s
user    0m1.196s
sys     0m1.672s

ubuntu@ubuntu:~/$ time ./target/release/blackwater -i www.baidu.com -p22-65535 -c 80000

 _      _
| |    | |
| |_   | |   __
| | |  | |  |  |
| _ |  |_|  |  |
Black Water
Asynchronous Port Scanner written in rust
https://github.com/dollarkillerx/blackwater

www.baidu.com:80
www.baidu.com:2000
www.baidu.com:443
www.baidu.com:5060

real    0m1.151s
user    0m0.654s
sys     0m0.697s

ubuntu@ubuntu:~/$ time ./target/release/blackwater -i www.bing.com -p22-65535 -c 80000

 _      _
| |    | |
| |_   | |   __
| | |  | |  |  |
| _ |  |_|  |  |
Black Water
Asynchronous Port Scanner written in rust
https://github.com/dollarkillerx/blackwater

www.bing.com:80
www.bing.com:443
www.bing.com:2000

real    0m1.079s
user    0m0.589s
sys     0m0.442s

ubuntu@ubuntu:~/$ time ./target/release/blackwater -i github.com -p22-65535 -c 80000

 _      _
| |    | |
| |_   | |   __
| | |  | |  |  |
| _ |  |_|  |  |
Black Water
Asynchronous Port Scanner written in rust
https://github.com/dollarkillerx/blackwater

github.com:2000
github.com:22
github.com:80
github.com:443
github.com:5060

real    0m1.137s
user    0m0.685s
sys     0m0.756s


Test Environment
Intel(R) Core(TM) i3-9100 CPU @ 3.60GHz
HHD
8G DDR4
```