# async & await 

## async함수

async 키워드는 function 앞에 위치한다. 

```js
async function f() {
  return 1;
}
```

async은 function 앞에 위치하며 해당 함수는 항상 프라미스를 반환한다. 

## await

await는 async함수 내에서만 동작한다. js가 await를 만다면 프라미스가 처리될 때까지 기다린다. 

**일반 함수에는 await를 사용하지 못한다.**

## 프라미스 체이닝을 async/await를 이용하여 바꾼다면?

* 먼저 .then호출을 await로 바꾼다. 
* function앞에 async를 붙여 await를 사용할 수 있도록 한다. 

만약 최상위 레벨에서 await를 이용하고 싶은 경우, 익명함수를 사용하자. 

await는 thenable객체를 받는다. 