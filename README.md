# mio-echo-server

A simple echo server using the frame mio of rust  
which can manipulate multiple tcp-client-connections.  

项目功能要求:  
使用 mio 监听 TCP 连接。  
支持多个客户端同时连接。  
每个客户端发送的数据，服务器原样发回（echo）。  
服务器能正确处理连接断开（客户端关闭）。  
可选：支持简单的广播（如“用户加入/离开”消息）。  
