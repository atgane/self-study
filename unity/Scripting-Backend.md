[https://docs.unity3d.com/kr/2020.3/Manual/scripting-backends.html](https://docs.unity3d.com/kr/2020.3/Manual/scripting-backends.html)

2020.3

# 스크립팅 백엔드

## 스크립팅 제한

### 사전 컴파일

일부 플랫폼은 런타임 코드 생성을 허용하지 않는데 대신 AOT를 미리 컴파일해야 한다. 

## System.Reflection.Emit

AOT 플랫폼은 System.Reflection.Emit 네임스페이스의 어떤 것도 구현할 수 없다. 커파일러가 Reflection을 통해 사용되는 코드가 런타임에 존재한다고 추론할 수 있는 한 System.Reflection의 나머지는 가능하다. (~~System.Reflection 쓰지 말자~~)

## Serialization

AOT플랫폼에서 Reflection의 사용으로 Serialization 또는 Deserialization에서 문제가 발생할 수 있다. 