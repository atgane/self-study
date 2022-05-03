# 프라미스 API

[https://ko.javascript.info/promise-api](https://ko.javascript.info/promise-api)

## Promise.all

여러 프라미스를 동시에 실행하고 모든 프라미스가 준비될 때까지 기다리는 경우 사용한다. 

다음 예는 동시로 fetch api를 가져오는 방법이다. 

```js
let urls = [
  'https://api.github.com/users/iliakan',
  'https://api.github.com/users/Violet-Bora-Lee',
  'https://api.github.com/users/jeresig'
];

// fetch를 사용해 url을 프라미스로 매핑합니다.
let requests = urls.map(url => fetch(url));

// Promise.all은 모든 작업이 이행될 때까지 기다립니다.
Promise.all(requests)
  .then(responses => responses.forEach(
    response => alert(`${response.url}: ${response.status}`)
  ));
```

Promise.all에 전달되는 프라미스 중 하나라도 거부되면, Promise.all이 반환하는 프라미스는 에러와 함께 거부된다. 

## Promise.allSettled

Promise.allSettled은 모든 프라미스가 처리될 때까지 기다린다. 반환되는 배열은 다음과 같은 요소를 갖는다. 

```js

const a = new Promise((resolve, reject) => {
  resolve(1)
})

const b = promise.allSettled([a, a])
[{status: ... , value: 1}, {}]

```


* 응답이 성공할 경우 – {status:"fulfilled", value:result}
* 에러가 발생한 경우 – {status:"rejected", reason:error}

## Promise.race

가장 먼저 처리되는 프라미스 결과를 반환한다. 