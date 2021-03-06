# 1. 네트워크 통신

네트워크를 통해 메세지, 파일을 송수신하는 방버을 알아보자. 

## 1.1. 네트워크 통신이란?

네트워크 통신을 하기 위해서 소켓이 필요하다. 네트워크는 컴퓨터와 컴퓨터가 연결되어 있을 때 네트워크가 형성되었다 한다. PC가 여러 개 있을 때 네트워크가 그냥 이루어지지 않는다. 다만 서로 연결되어 있다면 네트워크가 형성되어 데이터를 주고 받는 상황이 형성되었다는 것이다. 네트워크가 발전하며 인터넷 서비스가 이루어지고 www도 이루어졌는데 전부 네트워크 통신이라는 기반 하에서 인터넷과 같은 서비스가 발전했다. 컴퓨터와 컴퓨터가 통신하기 위해 통신장치가 필요하다. 전화를 하려면 전화기가 필요하고 회선이 필요할 것이다. 이 연결을 네트워크라 할 수 있고 전화기를 통신장치라 부르고 프로그래밍에서는 이를 소켓이라 부른다. 소켓은 네트워크 통신을 하기 위한 장치이다. 

통신을 하기 위한 조건은 네트워크가 유지되어야 한다는 것이다. 중간에 끊긴다면 통신은 불가능해진다. 통신이 이루어지기 위해서는 네트워크가 있어야 하고 주소가 있어야 한다. 이 주소를 IP주소라 한다. PC와 PC가 연결되어 있다고 하면 각 PC마다 주소가 있다. 또한 통신장치인 소켓이 필요하다. 이런 조건이 주어지면 언어의 API를 이용해서 통신을 할 수 있다. 

통신 방식에는 크게 ICP와 UDP가 있다. 특히 TCP/IP를 많이 들어보았을 것이다. TCP인 경우를 보자. 나와 상대방에 소켓이 있다고 하고 내가 상대방에게 데이터를 수신했을 때 친구집에 데이터가 정상적으로 도달을 했는지까지 점검하는 것이 TCP이다. UDP방식은 내가 그냥 일반적으로 점검하지 않고 보내기만 하는 것이다. 

TCP와 UDP의 차이는 송신확인의 유무이다. 따라서 TCP는 속도가 UDP보다 느리다. 주로 TCP를 사용하고 UDP는 큰 데이터를 사용할 때 이용한다. 

어떤 통신을 할 때 클라이언트와 서버라는 말이 많이 나온다. 클라이언트는 서버에게 요청을 하고 서버는 요청에 따라 피드백하여 답을 줄 수 있다. 클라이언트와 서버는 소켓이 반드시 있어야 한다. 클라이언트가 서버측에 요청하는 것을 request라 한다. request를 하면 서버측 소켓에 데이터가 전달되는데 앞단에 listener가 듣는다. 어떤 클라이언트가 어떤 요청을 하는지 듣고 데이터가 소켓으로 넘어간다. 서버는 요청받은 데이터를 가지고 작업을 한다. 데이터 베이스 연동이나 작업을 하고 클라이언트로 데이터를 피드백한다. 

이때 클라이언트와 서버가 통신하며 이동하는 데이터를 패킷이라 한다. 

네트워크와 관련된 사항과 클라이언트, 서버가 데이터를 주고받는데 리스너가 존재함까지 살펴보았다. 

## 1.2. 메세지 송수신

소켓을 이용한 메세지 송수신을 살펴보자. 클라이언트에서는 socket라이브러리를 임포트하고 서버의 아이피와 포트를 찾는다. 포트는 어떤 서버에 기능을 구현하여 소프트웨어를 탑재해 두었다면 서버의 주소가 있을 것이다. 서버 주소가 있다면 사용자들이 밖에서 들어올텐데 하나의 서버가 하나의 서비스만 하기에는 너무 아까울 것이다. 따라서 하나의 서버가 여러 서비스를 이용할 수 있도록 하나의 주소에서 여러 서비스로 기능을 나눈게 포트이다. 

포트는 약 65000개 정도 존재한다. 