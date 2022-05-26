[https://docs.unity3d.com/kr/2020.3/Manual/scripting-backends.html](https://docs.unity3d.com/kr/2020.3/Manual/scripting-backends.html)

2020.3

# 스크립팅 백엔드

## 스크립팅 제한

### 사전 컴파일

일부 플랫폼은 런타임 코드 생성을 허용하지 않는데 대신 AOT를 미리 컴파일해야 한다. 

### System.Reflection.Emit

AOT 플랫폼은 System.Reflection.Emit 네임스페이스의 어떤 것도 구현할 수 없다. 커파일러가 Reflection을 통해 사용되는 코드가 런타임에 존재한다고 추론할 수 있는 한 System.Reflection의 나머지는 가능하다. (~~System.Reflection 쓰지 말자같은데...~~)

### Serialization

AOT플랫폼에서 Reflection의 사용으로 Serialization 또는 Deserialization에서 문제가 발생할 수 있다. 타입이나 메서드가 Reflection을 통해서 사용된다면, AOT 컴파일러는 해당 타입이나 메서드의 코드 생성을 감지하지 못한다. 

### 제너릭 가상 메서드(~~Generic virtual methods?~~)

제너릭 메서드를 쓰는 경우, 컴파일러는 항상 장치에 실행되는 코드를 위해 추가적인 작업을 한다. 예를 들어, int형과 double형의 List를 쓰는 다른 코드가 필요할 수 있다. 만약 컴파일 때가 아닌 런타임 때 가상 메서드를 사용한다면, 컴파일러는 소스코드가 완벽하지 않은 위치에서 쉽게 런타임 코드 생성을 요청한다. 

아래 코드는 JIT 환경에서 올바르게 0을 출력한다. 

```cs
using UnityEngine;
using System;

public class AOTProblemExample : MonoBehaviour, IReceiver
{
    public enum AnyEnum 
    {
        Zero,
        One,
    }

    void Start() 
    {
        // Subtle trigger: The type of manager *must* be
        // IManager, not Manager, to trigger the AOT problem.
        IManager manager = new Manager();
        manager.SendMessage(this, AnyEnum.Zero);
    }

    public void OnMessage<T>(T value) 
    {
        Debug.LogFormat("Message value: {0}", value);
    }
}

public class Manager : IManager 
{
    public void SendMessage<T>(IReceiver target, T value) {
        target.OnMessage(value);
    }
}

public interface IReceiver
{
    void OnMessage<T>(T value);
}

public interface IManager 
{
    void SendMessage<T>(IReceiver target, T value);
}
```

그러나 IL2CPP AOT플랫폼에서 예외가 발생한다. 

```
ExecutionEngineException: Attempting to call method 'AOTProblemExample::OnMessage<AOTProblemExample+AnyEnum>' for which no ahead of time (AOT) code was generated.
at Manager.SendMessage[T] (IReceiver target, .T value) [0x00000] in <filename unknown>:0 
at AOTProblemExample.Start () [0x00000] in <filename unknown>:0 
```

마찬가지로 Mono도 예외가 발생한다. 

```
ExecutionEngineException: Attempting to JIT compile method 'Manager:SendMessage<AOTProblemExample/AnyEnum> (IReceiver,AOTProblemExample/AnyEnum)' while running with --aot-only.
at AOTProblemExample.Start () [0x00000] in <filename unknown>:0 
```

AOT 컴파일러는 AnyEnum의 T를 인자로 받는 제너릭 메서드 OnMessage의 코드 생성을 인지하지 못하고 이런 메서드를 스킵한다. 그래서 메서드가 호출됐을 때, 오류가 발생한다. 

오류 해결을 위해 컴파일러에게 강제로 코드를 생성하게 만들 수 있다. 아래와 같이 짜는 것이다. 

```cs
public void UsedOnlyForAOTCodeGeneration() 
{
    // IL2CPP needs only this line.
    OnMessage(AnyEnum.Zero);

    // Mono also needs this line. Note that we are
    // calling directly on the Manager, not the IManager interface.
    new Manager().SendMessage(null, AnyEnum.Zero);

    // Include an exception so we can be sure to know if this method is ever called.
    throw new InvalidOperationException("This method is used for AOT code generation only. Do not call it at runtime.");
}
```

컴파일러가 AnyEnum의 T로 적힌 OnMessage의 명시적 호출을 만났을 때, 런타임 실행 시 적절한 코드를 생성한다. UsedOnlyForAOTCodeGeneration 메서드는 호출될 필요가 없어지고, 단지 컴파일러가 인지할 수 있도록 존재하게 된다. 