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

### No threads

어떤 플랫폼은 쓰레드를 지원하지 않는다. 따라서 System.Threading 네임스페이스를 사용하는 코드는 런타임 오류가 뜰 수 있다. 또한, 닷넷 클래스 라이브러리의 몇몇 부분은 암묵적으로 쓰레드에 의존한다. System.Timers.Timer와 같이 종종 사용되는 예제의 클래스의 경우, 쓰레드에 의존되어 있다. 

### Exception filters

IL2CPP은 C# 예외 필터를 지원하지 않는다. 따라서 catch로 관리되는 코드를 적절히 변경해야 한다. 

### TypedReference

IL2CPP는 __makeref 타입의 System.TypedReference 키워드를 지원하지 않는다. 

### MarshalAs and FieldOffset attributes

IL2CPP은 런타임에서 MarshalAs와 FieldOffset 어트리뷰트의 reflection을 지원하지 않는다. 컴파일 시간에는 지원한다. 

### The dynamic keyword

IL2CPP은 C#의 dynamic 키워드를 지원하지 않는다. 

## Managed code stripping

코드 스트리핑: 빌드에서 사용되지 않는 코드를 줄이는 작업

IL2CPP의 경우, 빌드 시간을 단축할 수 있다. Managed code stripping은 프로젝트의 코드를 정적으로 분석하여, 클래스, 멤버, 도달할 수 없는 함수 부분을 감지하여 이루어진다. Optimization 섹션의 플레이어 세팅 -> Managed stripping Level 설정을 통해 유니티가 도달할 수 없는 코드를 얼마나 적극적으로 줄일지 제어할 수 있다. 

**reflection을 통해 동적으로 클래스나 멤버를 찾는 경우 코드 스트리핑 툴은 이를 감지하지 못하고 제거할 수 있다.** link.xml이나 Preserve 속성을 이용해서 이런 사실을 선언해야 한다. 

Player Settings -> Managed Stripping Level 옵션을 사용하여 유니티가 사용되지 않는 코드를 얼마나 적극적으로 줄일지 제어할 수 있다! 이 옵션의 기본값은 스크립팅 백엔드 설정에 따라 달라진다. 

* Disabled: 코드 제거되지 않음. Mono의 기본 레벨이다. IL2CPP의 경우 이 옵션을 선택할 수 없다.
* Low: 도달할 수 없는 코드를 대부분 제거하는 동시에 실제로 사용되는 코드를 스트리핑할 가능성은 최소화하는 규칙에 따라 코드를 제거한다. 크기 축소보다 가용성을 우선한다. IL2CPP의 기본옵션이다. 
* Medium: Low보다 적극적으로 코드제거를 한다. 
* 도달할 수 없는 코드를 최대한  제거하고 중간 스트리핑 레벨보다 작은 빌드를 생성한다. link.xml파일이나 Preserve속성을 추가해서 문제가 되는 섹션을 줄여야 한다. 빌드 시간이 오래걸린다. 

### Managed code stripping의 이해

유니티에서 프로젝트를 빌드하면 C# 코드를 CIL이라는 닷넷 바이트코드 형식으로 컴파일한다. CIL코드는 어셈블리로 패키징된다. 빌드 프로세스는 프로젝트에서 사용되는 어셈블리 코드의 분량과 관계없이 어셈블리 파일 전체를 포함한다(~~?~~).

### UnityLinker

유니티 빌드 프로세스는 UnityLinker라는 툴을 이용하여 Managed code stripping을 실행한다. UnityLinker는 유니티에서 작동하도록 커스터마이징된 Mono IL Linker이다. 

#### UnityLinker의 원리

UnityLinker는 프로젝트의 모든 어셈블리를 분석한다. 먼저 상위 레벨, 루트 타입, 메서드, 프로퍼티, 필드 등을 마킹한다. 씬의 게임 오브젝트에 추가한 MonoBehaviour 파생 클래스는 루트 타입이다. 다음으로 UnityLinker는 마킹한 루트를 분석하여 이러한 루트가 의존하는 모든 관리되는 코드를 식별하고 마킹한다. 정적 분석 후 마킹되지 않은 모든 잔류 코드를 실행 할 수 없는 것으로 파악하고 어셈블리에서 삭제한다. 

#### Reflection and code stripping

UnityLinker는 프로젝트 코드가 reflection을 통해 다른 코드를 참조하는 인스턴스를 감지하지 못할 수 있다. Managed Stripping Level을 높이면 이런 위험이 증가한다. 

UnityLinker는 일부 reflection 패턴을 감지하고 처리할 수 있다. 하지만 reflection을 본격적으로 사용하려면 UnityLinker에 변경하지 않을 클래스에 대한 힌트를 제공해야 한다. 이런 힌트를 link.xml 파일과 Preserve 속성을 통해 제공할 수 있다. 

#### Preserve 속성

UnityLinker의 코드 스트리핑을 막으려면 `[Preserve]`속성을 이용하자. 다음 리스트는 UnityLinker가 어떤 엔티티를 보존하는지 나타낸다. 

* Assembly: 모든 타입에 `[Preserve]` 속성을 적용한 것처럼 어셈블리의 모든 타입을 보존한다. 
```cs
using System;
  using UnityEngine.Scripting;

  [assembly: Preserve]

  namespace Example
  {
      public class Foo {}
  }
```
* Type: 타입과 기본 생성자를 보존한다. 
* Method: 선언 타입, 반환 타입, 메서드의 모든 인수 
* Property: 